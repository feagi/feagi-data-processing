use std::fmt;
use crate::error::{FeagiBytesError, FeagiDataProcessingError, GenomeError, IODataError};
use crate::genomic_structures::cortical_id::{CorticalID};
use crate::genomic_structures::{CorticalIOChannelIndex, SingleChannelDimensions};
use crate::genomic_structures::index_types::CorticalGroupingIndex;
use crate::io_data::IOTypeVariant;
macro_rules! define_io_cortical_types {
    (
        $cortical_io_type_enum_name:ident {
            $(
                $cortical_type_key_name:ident => {
                    friendly_debug_name: $display_name:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimensions: $channel_dimensions:expr,
                    io_variants: $io_variants:expr,
                }
            ),* $(,)?
        }
    ) => {

        // Type Enum
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $cortical_io_type_enum_name {
            $(
                $cortical_type_key_name
            ),*
        }

        impl std::fmt::Display for $cortical_io_type_enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ch = match self {
                    $(
                        Self::$cortical_type_key_name => $display_name
                    ),*
                };
                write!(f, "{}", ch)
            }
        }

        impl $cortical_io_type_enum_name {

            // Does no cortical ID checking
            pub(crate) fn get_type_from_bytes(id: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<CorticalType, FeagiDataProcessingError> {
                return Err(FeagiDataProcessingError::InternalError("Failed to map cortical ID to type!".into()));
                
                let mut id_0: [u8; CorticalID::CORTICAL_ID_LENGTH] = id.clone();
                //id_0.clone_from_slice(id);
                id_0[4] = 0;
                id_0[5] = 0;

                match &id_0 {
                    $(
                        $base_ascii => Ok((Self::$cortical_type_key_name).into())
                    ),*,
                    _ => return Err(FeagiDataProcessingError::InternalError("Failed to map cortical ID to type!".into()))
                }
                

            }

            pub fn to_cortical_id(&self, index: CorticalGroupingIndex) -> CorticalID {
                let (high, low) = u8_to_hex_char_u8(index.0);
                let mut output: [u8; CorticalID::CORTICAL_ID_LENGTH] =  match self {
                    $(
                        Self::$cortical_type_key_name => *$base_ascii
                    ),*,
                };
                output[4] = high;
                output[5] = low;
                CorticalID {bytes: output} // skip safety checks, we know this is fine
            }

            pub fn get_single_channel_dimensions(&self) -> SingleChannelDimensions {
                match self {
                    $(
                        Self::$cortical_type_key_name => $channel_dimensions.unwrap()
                    ),*
                }
            }

            pub fn get_possible_io_variants(&self) -> &[IOTypeVariant] {
                match self {
                    $(
                        Self::$cortical_type_key_name => &$io_variants
                    ),*
                }
            }
            
            pub fn verify_valid_io_variant(&self, checking: &IOTypeVariant) -> Result<(), FeagiDataProcessingError> {
                if !self.get_possible_io_variants().contains(checking){
                    return Err(IODataError::InvalidParameters(format!("IO Type Variant {} is invalid for Cortical IO Type {}!", checking.into(), self.to_string())).into());
                }
                Ok(())
            }

        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorticalType {
    Custom,
    Memory,
    Core(CoreCorticalType),
    Sensory(SensorCorticalType),
    Motor(MotorCorticalType),
}

impl fmt::Display for CorticalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            Self::Custom => write!(f, "Cortical Type: Custom"),
            Self::Memory => write!(f, "Cortical Type:  Memory"),
            Self::Core(c) => write!(f, "Cortical Type: Core ({})", c),
            Self::Sensory(s) => write!(f, "Cortical Type: Sensory ({})", s),
            Self::Motor(m) => write!(f, "Cortical Type: Motor ({})", m),
        }
    }
}

impl CorticalType {
    pub fn get_type_from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<CorticalType, FeagiDataProcessingError> {
        let start: u8 = bytes[0];
        match start { 
            b'c' => Ok(CorticalType::Custom),
            b'm' => Ok(CorticalType::Memory),
            b'_' => CoreCorticalType::get_type_from_bytes(bytes),
            b'i' => SensorCorticalType::get_type_from_bytes(bytes),
            b'o' => MotorCorticalType::get_type_from_bytes(bytes),
            _ => Err(handle_byte_id_mapping_fail(bytes))
        }
    }
    
    pub fn try_as_cortical_id(&self, io_cortical_index: CorticalGroupingIndex) -> Result<CorticalID, FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas can have arbritary Cortical IDs and thus cannot be convert to from type!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas can have arbritary Cortical IDs and thus cannot be convert to from type!".into()).into()),
            Self::Core(c) => {
                return Ok(CorticalID::new_core_cortical_area_id(*c)?)
            }
            Self::Sensory(s) => {
                return Ok(CorticalID::new_sensor_cortical_area_id(*s, io_cortical_index)?);
            }
            Self::Motor(m) => {
                return Ok(CorticalID::new_motor_cortical_area_id(*m, io_cortical_index)?);
            }
        }
        
    }

}


//region Core

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoreCorticalType {
    Death,
    Power
}

impl fmt::Display for CoreCorticalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            CoreCorticalType::Death => "Death",
            CoreCorticalType::Power => "Power"
        };
        write!(f, "{}", ch)
    }
}

impl From<CoreCorticalType> for CorticalType {
    fn from(core: CoreCorticalType) -> Self {
        CorticalType::Core(core)
    }
}

impl From<&CoreCorticalType> for CorticalType {
    fn from(core: &CoreCorticalType) -> Self {
        CorticalType::Core(*core)
    }
}

impl CoreCorticalType {

    pub fn to_cortical_id(&self) -> CorticalID {
        match self {
            Self::Death => CorticalID{bytes: *b"_death"},
            Self::Power => CorticalID{bytes: *b"_power"},
        }
    }

    pub(crate) fn get_type_from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<CorticalType, FeagiDataProcessingError> {
        match bytes {
            b"_death" => Ok(CoreCorticalType::Death.into()),
            b"_power" => Ok(CoreCorticalType::Power.into()),
            _ => Err(handle_byte_id_mapping_fail(bytes)),
        }
    }
}

//endregion

//region Sensor Cortical Area types

define_io_cortical_types!{
    SensorCorticalType {
        Infrared => {
            friendly_debug_name: "Infrared Sensor",
            base_ascii: b"iinf00",
            channel_dimensions: SingleChannelDimensions::new(Some(1), Some(1), Some(1)),
            io_variants: [IOTypeVariant::LinearNormalizedFloat],
        },
        ReverseInfrared => {
            friendly_debug_name: "Reverse Infrared Sensor",
            base_ascii: b"iiif00",
            channel_dimensions: SingleChannelDimensions::new(Some(1), Some(1), Some(1)),
            io_variants: [IOTypeVariant::LinearNormalizedFloat],
        },

        VisionCenterGray => {
            friendly_debug_name: "Center Vision Input (Grayscale)",
            base_ascii: b"ivcc00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionTopLeftGray => {
            friendly_debug_name: "Top Left Vision Input (Grayscale)",
            base_ascii: b"ivtl00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionTopMiddleGray => {
            friendly_debug_name: "Top Middle Vision Input (Grayscale)",
            base_ascii: b"ivtm00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionTopRightGray => {
            friendly_debug_name: "Top Right Vision Input (Grayscale)",
            base_ascii: b"ivtr00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionMiddleLeftGray => {
            friendly_debug_name: "Middle Left Vision Input (Grayscale)",
            base_ascii: b"ivml00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionMiddleRightGray => {
            friendly_debug_name: "Middle Right Vision Input (Grayscale)",
            base_ascii: b"ivmr00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionBottomLeftGray => {
            friendly_debug_name: "Bottom Left Vision Input (Grayscale)",
            base_ascii: b"ivbl00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionBottomMiddleGray => {
            friendly_debug_name: "Bottom Middle Vision Input (Grayscale)",
            base_ascii: b"ivbm00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionBottomRightGray => {
            friendly_debug_name: "Bottom Right Vision Input (Grayscale)",
            base_ascii: b"ivbr00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionCenterColor => {
            friendly_debug_name: "Center Vision Input (Color)",
            base_ascii: b"iVcc00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionTopLeftColor => {
            friendly_debug_name: "Top Left Vision Input (Color)",
            base_ascii: b"iVtl00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionTopMiddleColor => {
            friendly_debug_name: "Top Middle Vision Input (Color)",
            base_ascii: b"iVtm00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionTopRightColor => {
            friendly_debug_name: "Top Right Vision Input (Color)",
            base_ascii: b"iVtr00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionMiddleLeftColor => {
            friendly_debug_name: "Middle Left Vision Input (Color)",
            base_ascii: b"iVml00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionMiddleRightColor => {
            friendly_debug_name: "Middle Right Vision Input (Color)",
            base_ascii: b"iVmr00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionBottomLeftColor => {
            friendly_debug_name: "Bottom Left Vision Input (Color)",
            base_ascii: b"iVbl00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionBottomMiddleColor => {
            friendly_debug_name: "Bottom Middle Vision Input (Color)",
            base_ascii: b"iVbm00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
        VisionBottomRightColor => {
            friendly_debug_name: "Bottom Right Vision Input (Color)",
            base_ascii: b"iVbr00",
            channel_dimensions: SingleChannelDimensions::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame],
        },
    }    
}

impl From<SensorCorticalType> for CorticalType {
    fn from(input: SensorCorticalType) -> Self {
        CorticalType::Sensory(input)
    }
}

//endregion

//region Motor Cortical Area types

define_io_cortical_types!{
    MotorCorticalType {
        SpinningMotor => {
            friendly_debug_name: "Spinning Motor",
            base_ascii: b"omot00",
            channel_dimensions: SingleChannelDimensions::new(Some(1), Some(1), None),
            io_variants: [IOTypeVariant::LinearNormalizedFloat],
        },
    }    
}

impl From<MotorCorticalType> for CorticalType {
    fn from(input: MotorCorticalType) -> Self {
        CorticalType::Motor(input)
    }
}

//endregion

//region helpers
fn hex_chars_to_u8(high: char, low: char) -> Result<u8, GenomeError> {
    fn hex_value(c: char) -> Result<u8, GenomeError> {
        match c {
            '0'..='9' => Ok(c as u8 - b'0'),
            'a'..='f' => Ok(c as u8 - b'a' + 10),
            'A'..='F' => Ok(c as u8 - b'A' + 10),
            _ => Err(GenomeError::InvalidCorticalID(format!("Index of '{}' is not a valid hexadecimal!", c))),
        }
    }
    let hi = hex_value(high)?;
    let lo = hex_value(low)?;

    Ok((hi << 4) | lo)
}

fn u8_to_hex_char_u8(index: u8) -> (u8, u8) {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

    let high = HEX_CHARS[(index >> 4) as usize];
    let low = HEX_CHARS[(index & 0x0F) as usize];

    (high, low)
}

// Used when we know something is wrong, we just want the right error
fn handle_byte_id_mapping_fail(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> FeagiDataProcessingError {
    let as_string = String::from_utf8(bytes.to_vec());
    if as_string.is_err() {
        FeagiBytesError::UnableToDeserializeBytes("Unable to parse cortical ID as ASCII!".into()).into()
    }
    else {
        GenomeError::InvalidCorticalID(format!("Invalid cortical ID '{}'!", as_string.unwrap())).into()
    }
}
//endregion