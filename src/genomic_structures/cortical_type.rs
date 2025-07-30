use std::fmt;
use crate::error::{FeagiBytesError, FeagiDataProcessingError, GenomeError, IODataError};
use crate::genomic_structures::cortical_id::{CorticalID};
use crate::genomic_structures::{SingleChannelDimensionRange, SingleChannelDimensionsRequirements};
use crate::genomic_structures::index_types::CorticalGroupingIndex;
use crate::neuron_data::xyzp::NeuronCoderVariantType;
use crate::io_data::IOTypeVariant;

macro_rules! define_io_cortical_types {
    (
        $cortical_io_type_enum_name:ident {
            $(
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    encoder_type: $encoder_type:expr,
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

            pub fn get_channel_dimension_range(&self) -> SingleChannelDimensionRange {
                match self {
                    $(
                        Self::$cortical_type_key_name => $channel_dimension_range.unwrap()
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
                    return Err(IODataError::InvalidParameters(format!("IO Type Variant {} is invalid for Cortical IO Type {}!", checking, self.to_string())).into());
                }
                Ok(())
            }
            
            pub fn get_coder_type(&self) -> Result<NeuronCoderVariantType, FeagiDataProcessingError> {
                match self {
                    $(
                        Self::$cortical_type_key_name => Ok($encoder_type)
                    ),*
                }
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
    
    pub fn get_possible_io_variants(&self) -> &[IOTypeVariant] {
        match self {
            Self::Custom => &[],
            Self::Memory => &[],
            Self::Core(c) => &[],
            Self::Sensory(s) => s.get_possible_io_variants(),
            Self::Motor(m) => m.get_possible_io_variants(),
        }
    }
    
    pub fn verify_valid_io_variant(&self, checking: &IOTypeVariant) -> Result<(), FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas cannot have any valid IO Type Variant!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas cannot have any valid IO Type Variant!".into()).into()),
            Self::Core(c) => Err(IODataError::InvalidParameters("Core Cortical Areas cannot have any valid IO Type Variant!".into()).into()),
            Self::Sensory(s) => {s.verify_valid_io_variant(checking)},
            Self::Motor(m) => {m.verify_valid_io_variant(checking)},
        }
    }

    pub fn try_get_channel_size_boundaries(&self) -> Result<SingleChannelDimensionRange, FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas do not have channels!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas do not have channels!".into()).into()),
            Self::Core(c) => Ok(c.get_channel_size_boundaries()),
            Self::Sensory(s) => Ok(s.get_channel_size_boundaries()),
            Self::Motor(m) => Ok(m.get_channel_size_boundaries()),
        }
    }
    
    pub fn try_get_coder_type(&self) -> Result<NeuronCoderVariantType, FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas do not have coders!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas do not have coders!".into()).into()),
            Self::Core(_) => Err(IODataError::InvalidParameters("Core Cortical Areas do not have coders!".into()).into()),
            Self::Sensory(s) => s.get_coder_type(),
            Self::Motor(m) => m.get_coder_type(),
        }
    }
    
    
    pub fn is_type_core(&self) -> bool {
        match self {
            Self::Core(_) => true,
            _ => false
        }
    }

    pub fn is_type_sensor(&self) -> bool {
        match self {
            Self::Sensory(_) => true,
            _ => false
        }
    }

    pub fn is_type_motor(&self) -> bool {
        match self {
            Self::Motor(_) => true,
            _ => false
        }
    }

    pub fn is_type_custom(&self) -> bool {
        match self {
            Self::Custom => true,
            _ => false
        }
    }

    pub fn is_type_memory(&self) -> bool {
        match self {
            Self::Memory => true,
            _ => false
        }
    }
    
    pub fn verify_is_core(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_core() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Core!".into()).into())
        }
        Ok(())
    }
    pub fn verify_is_sensor(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_sensor() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Sensor!".into()).into())
        }
        Ok(())
    }
    
    pub fn verify_is_motor(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_motor() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Motor!".into()).into())
        }
        Ok(())
    }

    pub fn verify_is_custom(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_custom() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Custom!".into()).into())
        }
        Ok(())
    }

    pub fn verify_is_memory(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_memory() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Memory!".into()).into())
        }
        Ok(())
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
    
    pub fn get_channel_dimension_range(&self)  -> SingleChannelDimensionRange {
        match self {
            CoreCorticalType::Death => SingleChannelDimensionRange::new(1..2, 1..2, 1..2).unwrap(),
            CoreCorticalType::Power =>SingleChannelDimensionRange::new(1..2, 1..2, 1..2).unwrap()
        }
    }
}

//endregion

//region Sensor Cortical Area types

define_io_cortical_types!{
    sensor_definition!{sensor_definition!{}}
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
        RotoryMotor => {
            friendly_name: "Rotory Motor",
            base_ascii: b"omot00",
            channel_dimension_range: : ChannelDimensionRange::new(1..2, 1..2, 1..u32::MAX).unwrap(),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::NormalizedM1To1F32_PSPBirdirectionalDivided,
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
