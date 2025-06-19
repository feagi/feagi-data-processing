use std::fmt;
use crate::error::DataProcessingError;

macro_rules! define_indexed_cortical_enum_and_cortical_types {
    (
        $cortical_type_enum_name:ident {
            $(
                $cortical_type_key:ident => {
                    friendly_debug_name: $display_name:expr,
                    base_ascii: $base_ascii:expr
                }
            ),* $(,)?
        }
    ) => {

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $cortical_type_enum_name {
            $(
                $cortical_type_key
            ),*
        }

        impl std::fmt::Display for $cortical_type_enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ch = match self {
                    $(
                        Self::$cortical_type_key => $display_name
                    ),*
                };
                write!(f, "{}", ch)
            }
        }

        impl $cortical_type_enum_name {

            pub fn to_string_with_index(&self, index: u8) -> String {
                format!("{} (Index: {})", self, index)
            }

            fn from_bytes(bytes: &[u8; CORTICAL_ID_LENGTH]) -> Result<(Self, u8), DataProcessingError> {
                // We assume that the structure is all ASCII, and the first letter is correct
                let mut comparing_base_slice: [u8; CORTICAL_ID_LENGTH] = *b"000000";
                comparing_base_slice[0..4].copy_from_slice(&bytes[0..4]);

                match &comparing_base_slice {
                    $(
                        $base_ascii => {
                            let index: u8 = hex_chars_to_u8(bytes[4] as char, bytes[5] as char)?;
                            let cortical_type = Self::$cortical_type_key;
                            Ok((cortical_type, index))
                        }
                    ),*,
                    _ => {
                        let bytes_as_ascii = safe_bytes_to_string(bytes);
                        return Err(DataProcessingError::InvalidCorticalID(format!("Given ID '{}' is not a known {}!", bytes_as_ascii, stringify!($cortical_type_enum_name))));
                    },
                }
            }

            fn to_bytes(&self, index: u8) -> [u8; CORTICAL_ID_LENGTH] {
                match self {
                    $(
                        Self::$cortical_type_key => {
                            let mut ascii: [u8; CORTICAL_ID_LENGTH] = *$base_ascii;
                            let (upper, lower) = u8_to_hex_char_u8(index);
                            ascii[4] = upper;
                            ascii[5] = lower;
                            ascii
                        }
                    ),*
                }
            }
        }
    }
}

//region Cortical IDs
pub const CORTICAL_ID_LENGTH: usize = 6;

// Public wrapper that enforces safety checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorticalID(CorticalIDInternal);

impl CorticalID {
    pub fn new_custom_cortical_area_id(desired_id_string: String) -> Result<Self, DataProcessingError> {
        CorticalIDInternal::verify_all_universal_id_rules(&desired_id_string)?;
        let bytes = desired_id_string.as_bytes();
        let bytes: &[u8; CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
        if bytes[0] != b'c' {
            return Err(DataProcessingError::InvalidCorticalID(format!("A custom cortical area ID must start with 'c'! Cortical area given: {}", desired_id_string)));
        }
        Ok(CorticalID(CorticalIDInternal::Custom(*bytes)))
    }

    pub fn new_memory_cortical_area_id(desired_id_string: String) -> Result<Self, DataProcessingError> {
        CorticalIDInternal::verify_all_universal_id_rules(&desired_id_string)?;
        let bytes = desired_id_string.as_bytes();
        let bytes: &[u8; CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
        if bytes[0] != b'm' {
            return Err(DataProcessingError::InvalidCorticalID(format!("A memory cortical area ID must start with 'm'! Cortical area given: {}", desired_id_string)));
        }
        Ok(CorticalID(CorticalIDInternal::Memory(*bytes)))
    }

    pub fn new_core_cortical_area_id(core_type: CoreCorticalType) -> Result<Self, DataProcessingError> {
        Ok(CorticalID(CorticalIDInternal::Core(core_type)))
    }

    pub fn new_input_cortical_area_id(input_type: InputCorticalType, input_index: u8) -> Result<Self, DataProcessingError> {
        Ok(CorticalID(CorticalIDInternal::Input((input_type, input_index))))
    }

    pub fn new_output_cortical_area_id(output_type: OutputCorticalType, output_index: u8) -> Result<Self, DataProcessingError> {
        Ok(CorticalID(CorticalIDInternal::Output((output_type, output_index))))
    }
    
    pub fn create_ordered_cortical_areas_for_segmented_vision(camera_index: u8, is_grayscale: bool) -> [CorticalID; 9] {
        if is_grayscale {
            return [
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionCenterGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionBottomLeftGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionMiddleLeftGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionTopLeftGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionTopMiddleGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionTopRightGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionMiddleRightGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionBottomRightGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionBottomMiddleGray, camera_index))),
            ]
        }
        else {
            return [ // TODO Shouldn't these all be in color?
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionCenterColor, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionBottomLeftGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionMiddleLeftGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionTopLeftGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionTopMiddleGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionTopRightGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionMiddleRightGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionBottomRightGray, camera_index))),
                CorticalID(CorticalIDInternal::Input((InputCorticalType::VisionBottomMiddleGray, camera_index))),
            ]
        }
    }

    pub fn from_bytes(bytes: &[u8; CORTICAL_ID_LENGTH]) -> Result<Self, DataProcessingError> {
        CorticalIDInternal::from_bytes(bytes).map(CorticalID)
    }

    pub fn from_ascii_string(string: &str) -> Result<Self, DataProcessingError> {
        CorticalIDInternal::from_ascii_string(string).map(CorticalID)
    }

    pub fn to_bytes(&self) -> [u8; CORTICAL_ID_LENGTH] {
        self.0.to_bytes()
    }

    pub fn write_bytes_at(&self, target: &mut [u8; CORTICAL_ID_LENGTH]) -> Result<(), DataProcessingError> {
        self.0.write_bytes_at(target)
    }

    pub fn to_identifier_ascii_string(&self) -> String {
        self.0.to_identifier_ascii_string()
    }

    // Internal method to access the inner value when needed
    pub(crate) fn inner(&self) -> &CorticalIDInternal {
        &self.0
    }
}

impl fmt::Display for CorticalID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

mod hidden_cortical_id_internals {
    use std::fmt;
    use crate::error::DataProcessingError;
    use crate::genome_definitions::identifiers::{safe_bytes_to_string, CORTICAL_ID_LENGTH};
    use crate::genome_definitions::identifiers::CoreCorticalType;
    use crate::genome_definitions::identifiers::InputCorticalType;
    use crate::genome_definitions::identifiers::OutputCorticalType;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CorticalIDInternal {
        Custom([u8; CORTICAL_ID_LENGTH]),
        Memory([u8; CORTICAL_ID_LENGTH]),
        Core(CoreCorticalType),
        Input((InputCorticalType, u8)),
        Output((OutputCorticalType, u8)),
    }

    impl fmt::Display for CorticalIDInternal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let ch = match self {
                CorticalIDInternal::Custom(v) => {
                    format!("Custom (inter) Cortical Area of ID: {}", safe_bytes_to_string(v) )
                }
                CorticalIDInternal::Memory(v) => {
                    format!("Memory Cortical Area of ID: {}", safe_bytes_to_string(v) )
                }
                CorticalIDInternal::Core(v) => {
                    format!("Core '{}' Cortical Area", v.to_string() )
                }
                CorticalIDInternal::Input(v) => {
                    format!("Input '{}' Cortical Area", v.0.to_string_with_index(v.1) )
                }
                CorticalIDInternal::Output(v) => {
                    format!("Output '{}' Cortical Area", v.0.to_string_with_index(v.1) )
                }
            };
            write!(f, "{}", ch)
        }
    }

    impl CorticalIDInternal {
        pub fn from_bytes(bytes: &[u8; CORTICAL_ID_LENGTH]) -> Result<Self, DataProcessingError> {
            if !bytes.iter().all(|&b| b.is_ascii()) {
                return Err(DataProcessingError::InvalidCorticalID("Cortical ID must contain only ASCII characters!".into()));
            }
            let first_char = bytes[0];
            match first_char {
                b'_' => CoreCorticalType::from_bytes(*bytes).map(Self::Core),
                b'c' => Ok(CorticalIDInternal::Custom(*bytes)),
                b'm' => Ok(CorticalIDInternal::Memory(*bytes)),
                b'i' => InputCorticalType::from_bytes(bytes).map(Self::Input),
                b'o' => OutputCorticalType::from_bytes(bytes).map(Self::Output),
                _ => Err(DataProcessingError::InvalidCorticalID(format!("Invalid cortical ID: {}", safe_bytes_to_string(bytes)).into())),
            }
        }

        pub fn from_ascii_string(string: &str) -> Result<Self, DataProcessingError> {
            if string.len() != CORTICAL_ID_LENGTH {
                return Err(DataProcessingError::InvalidInputBounds("Cortical Area ID Incorrect Length!".into()));
            }
            let bytes: &[u8] = string.as_bytes();
            let bytes: &[u8; CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
            CorticalIDInternal::from_bytes(&bytes) // further checks handled here
        }

        pub fn to_bytes(&self) -> [u8; CORTICAL_ID_LENGTH] {
            match self {
                CorticalIDInternal::Core(v) => {*v.to_bytes()}
                CorticalIDInternal::Custom(v) => *v,
                CorticalIDInternal::Memory(v) => *v,
                CorticalIDInternal::Input(v) => v.0.to_bytes(v.1),
                CorticalIDInternal::Output(v) => v.0.to_bytes(v.1),
            }
        }

        pub fn write_bytes_at(&self, target: &mut [u8; CORTICAL_ID_LENGTH]) -> Result<(), DataProcessingError> {
            let bytes = self.to_bytes();
            target.copy_from_slice(&bytes);
            Ok(())
        }

        pub fn to_identifier_ascii_string(&self) -> String {
            let bytes = self.to_bytes();
            safe_bytes_to_string(&bytes)
        }

        pub fn verify_all_universal_id_rules(string: &String)  -> Result<(), DataProcessingError> {
            CorticalIDInternal::verify_input_length(string)?;
            CorticalIDInternal::verify_input_ascii(string)?;
            CorticalIDInternal::verify_allowed_characters(string)?;
            Ok(())
        }

        fn verify_input_length(string: &String) -> Result<(), DataProcessingError> {
            if string.len() != CORTICAL_ID_LENGTH {
                return Err(DataProcessingError::InvalidCorticalID(format!("A cortical ID must have a length of {}! Given cortical ID '{}' is not!", CORTICAL_ID_LENGTH, string)).into());
            }
            Ok(())
        }

        fn verify_input_ascii(string: &String) -> Result<(), DataProcessingError> {
            if !string.is_ascii() {
                return Err(DataProcessingError::InvalidCorticalID(format!("A cortical ID must be entirely ASCII! Given cortical ID '{}' is not!", string)).into());
            }
            Ok(())
        }

        fn verify_allowed_characters(string: &String) -> Result<(), DataProcessingError> {
            if !string.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(DataProcessingError::InvalidCorticalID(format!("A cortical ID must be made only of alphanumeric characters and underscores! Given cortical ID '{}' is not!", string)).into());
            }
            Ok(())
        }
    }
}

// Make CorticalIDInternal available to the wrapper but not publicly
use hidden_cortical_id_internals::CorticalIDInternal;
//endregion

//region Cortical Types

// Cortical Input Types
define_indexed_cortical_enum_and_cortical_types! {
    InputCorticalType {
        Infrared => {
            friendly_debug_name: "Infrared Sensor",
            base_ascii: b"iinf00"
        },
        ReverseInfrared => {
            friendly_debug_name: "Reverse Infrared Sensor",
            base_ascii: b"iiif00"
        },
        Proximity => {
            friendly_debug_name: "Proximity Sensor",
            base_ascii: b"ipro00"
        },
        DigitalGPIO => {
            friendly_debug_name: "Digital GPIO input",
            base_ascii: b"igpd00"
        },
        AnalogGPIO=> {
            friendly_debug_name: "Analog GPIO Input",
            base_ascii: b"igpa00"
        },
        Accelerometer => {
            friendly_debug_name: "Accelerometer Input",
            base_ascii: b"iacc00"
        },
        Gyro => {
            friendly_debug_name: "Gyro Input",
            base_ascii: b"igyr00"
        },
        Euler => {
            friendly_debug_name: "Euler Input",
            base_ascii: b"ieul00"
        },
        Shock => {
            friendly_debug_name: "Shock Input",
            base_ascii: b"isho00"
        },
        Battery => {
            friendly_debug_name: "Battery Input",
            base_ascii: b"ibat00"
        },
        Compass => {
            friendly_debug_name: "Compass Input",
            base_ascii: b"icom00"
        },
        VisionCenterGray => {
            friendly_debug_name: "Center Vision Input (Grayscale)",
            base_ascii: b"ivcc00"
        },
        VisionTopLeftGray => {
            friendly_debug_name: "Top Left Vision Input (Grayscale)",
            base_ascii: b"ivtl00"
        },
        VisionTopMiddleGray => {
            friendly_debug_name: "Top Middle Vision Input (Grayscale)",
            base_ascii: b"ivtm00"
        },
        VisionTopRightGray => {
            friendly_debug_name: "Top Right Vision Input (Grayscale)",
            base_ascii: b"ivtr00"
        },
        VisionMiddleLeftGray => {
            friendly_debug_name: "Middle Left Vision Input (Grayscale)",
            base_ascii: b"ivml00"
        },
        VisionMiddleRightGray => {
            friendly_debug_name: "Middle Right Vision Input (Grayscale)",
            base_ascii: b"ivmr00"
        },
        VisionBottomLeftGray => {
            friendly_debug_name: "Bottom Left Vision Input (Grayscale)",
            base_ascii: b"ivbl00"
        },
        VisionBottomMiddleGray => {
            friendly_debug_name: "Bottom Middle Vision Input (Grayscale)",
            base_ascii: b"ivbm00"
        },
        VisionBottomRightGray => {
            friendly_debug_name: "Bottom Right Vision Input (Grayscale)",
            base_ascii: b"ivbr00"
        },
        VisionCenterColor => {
            friendly_debug_name: "Center Vision Input (Color)",
            base_ascii: b"iVcc00"
        },
        VisionTopLeftColor => {
            friendly_debug_name: "Top Left Vision Input (Color)",
            base_ascii: b"iVtl00"
        },
        VisionTopMiddleColor => {
            friendly_debug_name: "Top Middle Vision Input (Color)",
            base_ascii: b"iVtm00"
        },
        VisionTopRightColor => {
            friendly_debug_name: "Top Right Vision Input (Color)",
            base_ascii: b"iVtr00"
        },
        VisionMiddleLeftColor => {
            friendly_debug_name: "Middle Left Vision Input (Color)",
            base_ascii: b"iVml00"
        },
        VisionMiddleRightColor => {
            friendly_debug_name: "Middle Right Vision Input (Color)",
            base_ascii: b"iVmr00"
        },
        VisionBottomLeftColor => {
            friendly_debug_name: "Bottom Left Vision Input (Color)",
            base_ascii: b"iVbl00"
        },
        VisionBottomMiddleColor => {
            friendly_debug_name: "Bottom Middle Vision Input (Color)",
            base_ascii: b"iVbm00"
        },
        VisionBottomRightColor => {
            friendly_debug_name: "Bottom Right Vision Input (Color)",
            base_ascii: b"iVbr00"
        },
        Miscellaneous => {
            friendly_debug_name: "Miscellaneous",
            base_ascii: b"imis00"
        },
        ServoPosition => {
            friendly_debug_name: "Servo Position",
            base_ascii: b"ispo00"
        },
        ServoMotion => {
            friendly_debug_name: "Servo Motion",
            base_ascii: b"ismo00"
        },
        IDTrainer => {
            friendly_debug_name: "ID Trainer",
            base_ascii: b"iidt00"
        },
        Pressure => {
            friendly_debug_name: "Pressure",
            base_ascii: b"ipre00"
        },
        Lidar => {
            friendly_debug_name: "Lidar",
            base_ascii: b"ilid00"
        },
        Audio => {
            friendly_debug_name: "Audio",
            base_ascii: b"iear00"
        }
    }
}

// Cortical Output Types
define_indexed_cortical_enum_and_cortical_types! {
    OutputCorticalType {
        SpinningMotor => {
            friendly_debug_name: "Spinning Motor",
            base_ascii: b"omot00"
        },
        ServoMotion => {
            friendly_debug_name: "Servo (Delta Motion)",
            base_ascii: b"osmo00"
        },
        ServoPosition => {
            friendly_debug_name: "Servo (Absolute Position)",
            base_ascii: b"ospo00"
        },
        MotionControl => {
            friendly_debug_name: "Motion Control",
            base_ascii: b"omcl00"
        },
        Battery => {
            friendly_debug_name: "Battery",
            base_ascii: b"pbat00"
        },
    }
}

// Cortical Core Types
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

impl CoreCorticalType {

    fn from_bytes(bytes: [u8; CORTICAL_ID_LENGTH]) -> Result<CoreCorticalType, DataProcessingError> {
        match &bytes {
            b"_death" => Ok(CoreCorticalType::Death),
            b"_power" => Ok(CoreCorticalType::Power),
            _ => Err(handle_byte_id_mapping_fail(bytes)),
        }
    }

    fn to_bytes(&self) -> &'static [u8; CORTICAL_ID_LENGTH] {
        match self {
            CoreCorticalType::Death => b"_death",
            CoreCorticalType::Power => b"___pwr"
        }
    }
}

//endregion

//region Local Helper Functions
fn hex_chars_to_u8(high: char, low: char) -> Result<u8, DataProcessingError> {
    fn hex_value(c: char) -> Result<u8, DataProcessingError> {
        match c {
            '0'..='9' => Ok(c as u8 - b'0'),
            'a'..='f' => Ok(c as u8 - b'a' + 10),
            'A'..='F' => Ok(c as u8 - b'A' + 10),
            _ => Err(DataProcessingError::InvalidCorticalID(format!("Index of '{}' is not a valid hexadecimal!", c))),
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

// This function assumes that we know the bytes are valid ASCII
fn safe_bytes_to_string(bytes: &[u8; CORTICAL_ID_LENGTH]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

// Used when we know something is wrong, we just want the right error
fn handle_byte_id_mapping_fail(bytes: [u8; CORTICAL_ID_LENGTH]) -> DataProcessingError {
    let as_string = String::from_utf8(bytes.to_vec());
    if as_string.is_err() {
        DataProcessingError::InvalidCorticalID("Unable to parse cortical ID as ASCII!".into())
    }
    else {
        DataProcessingError::InvalidCorticalID(format!("Invalid cortical ID '{}'!", as_string.unwrap()))
    }
}
//endregion