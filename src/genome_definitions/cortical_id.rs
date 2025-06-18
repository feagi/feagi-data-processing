use std::fmt;
use crate::error::DataProcessingError;

pub const CORTICAL_ID_LENGTH: usize = 6;

macro_rules! define_indexed_cortical_enum {
    (
        $enum_name:ident {
            $(
                $variant:ident => {
                    name: $display_name:expr,
                    base_ascii: $base_ascii:expr
                }
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[derive(Hash)]
pub enum $enum_name {
            $(
                $variant(u8)
            ),*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ch = match self {
                    $(
                        Self::$variant(v) => format!("{} Index: {}", $display_name, v)
                    ),*
                };
                write!(f, "{}", ch)
            }
        }

        impl $enum_name {
            pub fn from_bytes(mut bytes: [u8; CORTICAL_ID_LENGTH]) -> Result<Self, DataProcessingError> {
                let index = hex_chars_to_u8(bytes[4] as char, bytes[5] as char)?;
                bytes[4] = 0;
                bytes[5] = 0;
                match &bytes {
                    $(
                        $base_ascii => Ok(Self::$variant(index))
                    ),*,
                    _ => Err(handle_byte_id_mapping_fail(bytes)),
                }
            }

            pub fn to_bytes(&self) -> [u8; CORTICAL_ID_LENGTH] {
                match self {
                    $(
                        Self::$variant(v) => {
                            let mut output: [u8; CORTICAL_ID_LENGTH] = *$base_ascii;
                            let (high, low) = u8_to_hex_char_u8(*v);
                            output[4] = high;
                            output[5] = low;
                            output
                        }
                    ),*
                }
            }
        }
    };
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Hash)]
pub enum CorticalID {
    Custom([u8; CORTICAL_ID_LENGTH]),
    Memory([u8; CORTICAL_ID_LENGTH]),
    Core(CoreCorticalID),
    Input(InputCorticalID),
    Output(OutputCorticalID),
}

impl fmt::Display for CorticalID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            CorticalID::Custom(v) => {
                format!("Custom (inter) Cortical Area of ID: {}", safe_bytes_to_string(v) )
            }
            CorticalID::Memory(v) => {
                format!("Memory Cortical Area of ID: {}", safe_bytes_to_string(v) )
            }
            CorticalID::Core(v) => {
                format!("Core '{}' Cortical Area", v.to_string() )
            }
            CorticalID::Input(v) => {
                format!("Input '{}' Cortical Area", v.to_string() )
            }
            CorticalID::Output(v) => {
                format!("Output '{}' Cortical Area", v.to_string() )
            }
        };
        write!(f, "{}", ch)
    }
}

impl CorticalID {

    pub fn new_custom_cortical_area_id(desired_id_string: String) -> Result<Self, DataProcessingError> {
        CorticalID::verify_all_universal_id_rules(&desired_id_string)?;
        let bytes = desired_id_string.as_bytes();
        let bytes: &[u8; CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
        if bytes[0] != b'c' {
            return Err(DataProcessingError::InvalidCorticalID(format!("A custom cortical area ID must start with 'c'! Cortical area given: {}", desired_id_string)));
        }
        Ok(CorticalID::Custom(*bytes))
    }

    pub fn new_memory_cortical_area_id(desired_id_string: String) -> Result<Self, DataProcessingError> {
        CorticalID::verify_all_universal_id_rules(&desired_id_string)?;
        let bytes = desired_id_string.as_bytes();
        let bytes: &[u8; CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
        if bytes[0] != b'm' {
            return Err(DataProcessingError::InvalidCorticalID(format!("A memory cortical area ID must start with 'm'! Cortical area given: {}", desired_id_string)));
        }
        Ok(CorticalID::Memory(*bytes))
    }
    
    

    pub fn from_bytes(bytes: &[u8; CORTICAL_ID_LENGTH]) -> Result<Self, DataProcessingError> {
        if !bytes.iter().all(|&b| b.is_ascii()) {
            return Err(DataProcessingError::InvalidCorticalID("Cortical ID must contain only ASCII characters!".into()));
        }
        let first_char = bytes[0];
        match first_char {
            b'_' => CoreCorticalID::from_bytes(*bytes).map(Self::Core),
            b'c' => Ok(CorticalID::Custom(*bytes)),
            b'm' => Ok(CorticalID::Memory(*bytes)),
            b'i' => InputCorticalID::from_bytes(*bytes).map(Self::Input),
            b'o' => OutputCorticalID::from_bytes(*bytes).map(Self::Output),
            _ => Err(DataProcessingError::InvalidCorticalID(format!("Invalid cortical ID: {}", safe_bytes_to_string(bytes)).into())),
        }
    }

    pub fn from_ascii_string(string: &str) -> Result<Self, DataProcessingError> {
        if string.len() != CORTICAL_ID_LENGTH {
            return Err(DataProcessingError::InvalidInputBounds("Cortical Area ID Incorrect Length!".into()));
        }
        let bytes: &[u8] = string.as_bytes();
        let mut inner = [0u8; CORTICAL_ID_LENGTH]; // TODO there has to be a better way than this
        inner.copy_from_slice(bytes);
        CorticalID::from_bytes(&inner)
    }

    pub fn to_bytes(&self) -> [u8; CORTICAL_ID_LENGTH] {
        match self {
            CorticalID::Core(v) => {*v.to_bytes()}
            CorticalID::Custom(v) => *v,
            CorticalID::Memory(v) => *v,
            CorticalID::Input(v) => v.to_bytes(),
            CorticalID::Output(v) => v.to_bytes(),
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

    fn verify_all_universal_id_rules(string: &String)  -> Result<(), DataProcessingError> {
        CorticalID::verify_input_length(string)?;
        CorticalID::verify_input_ascii(string)?;
        CorticalID::verify_allowed_characters(string)?;
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


//region Cortical ID Types

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Hash)]
pub enum CoreCorticalID {
    Death,
    Power
}

impl fmt::Display for CoreCorticalID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            CoreCorticalID::Death => "Death",
            CoreCorticalID::Power => "Power"
        };
        write!(f, "{}", ch)
    }
}

impl CoreCorticalID {

    fn from_bytes(bytes: [u8; CORTICAL_ID_LENGTH]) -> Result<CoreCorticalID, DataProcessingError> {
        match &bytes {
            b"_death" => Ok(CoreCorticalID::Death),
            b"_power" => Ok(CoreCorticalID::Power),
            _ => Err(handle_byte_id_mapping_fail(bytes)),
        }
    }
    
    fn to_bytes(&self) -> &'static [u8; CORTICAL_ID_LENGTH] {
        match self {
            CoreCorticalID::Death => b"_death",
            CoreCorticalID::Power => b"___pwr"
        }
    }
}

// Inputs
define_indexed_cortical_enum! {
    InputCorticalID {
        Infrared => {
            name: "Infrared Sensor",
            base_ascii: b"iinf00"
        },
        ReverseInfrared => {
            name: "Reverse Infrared Sensor",
            base_ascii: b"iiif00"
        },
        Proximity => {
            name: "Proximity Sensor",
            base_ascii: b"ipro00"
        },
        DigitalGPIO => {
            name: "Digital GPIO input",
            base_ascii: b"igpd00"
        },
        AnalogGPIO=> {
            name: "Analog GPIO Input",
            base_ascii: b"igpa00"
        },
        Accelerometer => {
            name: "Accelerometer Input",
            base_ascii: b"iacc00"
        },
        Gyro => {
            name: "Gyro Input",
            base_ascii: b"igyr00"
        },
        Euler => {
            name: "Euler Input",
            base_ascii: b"ieul00"
        },
        Shock => {
            name: "Shock Input",
            base_ascii: b"isho00"
        },
        Battery => {
            name: "Battery Input",
            base_ascii: b"ibat00"
        },
        Compass => {
            name: "Compass Input",
            base_ascii: b"icom00"
        },
        VisionCenterGray => {
            name: "Center Vision Input (Grayscale)",
            base_ascii: b"ivcc00"
        },
        VisionTopLeftGray => {
            name: "Top Left Vision Input (Grayscale)",
            base_ascii: b"ivtl00"
        },
        VisionTopMiddleGray => {
            name: "Top Middle Vision Input (Grayscale)",
            base_ascii: b"ivtm00"
        },
        VisionTopRightGray => {
            name: "Top Right Vision Input (Grayscale)",
            base_ascii: b"ivtr00"
        },
        VisionMiddleLeftGray => {
            name: "Middle Left Vision Input (Grayscale)",
            base_ascii: b"ivml00"
        },
        VisionMiddleRightGray => {
            name: "Middle Right Vision Input (Grayscale)",
            base_ascii: b"ivmr00"
        },
        VisionBottomLeftGray => {
            name: "Bottom Left Vision Input (Grayscale)",
            base_ascii: b"ivbl00"
        },
        VisionBottomMiddleGray => {
            name: "Bottom Middle Vision Input (Grayscale)",
            base_ascii: b"ivbm00"
        },
        VisionBottomRightGray => {
            name: "Bottom Right Vision Input (Grayscale)",
            base_ascii: b"ivbr00"
        },
        VisionCenterColor => {
            name: "Center Vision Input (Color)",
            base_ascii: b"iVcc00"
        },
        VisionTopLeftColor => {
            name: "Top Left Vision Input (Color)",
            base_ascii: b"iVtl00"
        },
        VisionTopMiddleColor => {
            name: "Top Middle Vision Input (Color)",
            base_ascii: b"iVtm00"
        },
        VisionTopRightColor => {
            name: "Top Right Vision Input (Color)",
            base_ascii: b"iVtr00"
        },
        VisionMiddleLeftColor => {
            name: "Middle Left Vision Input (Color)",
            base_ascii: b"iVml00"
        },
        VisionMiddleRightColor => {
            name: "Middle Right Vision Input (Color)",
            base_ascii: b"iVmr00"
        },
        VisionBottomLeftColor => {
            name: "Bottom Left Vision Input (Color)",
            base_ascii: b"iVbl00"
        },
        VisionBottomMiddleColor => {
            name: "Bottom Middle Vision Input (Color)",
            base_ascii: b"iVbm00"
        },
        VisionBottomRightColor => {
            name: "Bottom Right Vision Input (Color)",
            base_ascii: b"iVbr00"
        },
        Miscellaneous => {
            name: "Miscellaneous",
            base_ascii: b"imis00"
        },
        ServoPosition => {
            name: "Servo Position",
            base_ascii: b"ispo00"
        },
        ServoMotion => {
            name: "Servo Motion",
            base_ascii: b"ismo00"
        },
        IDTrainer => {
            name: "ID Trainer",
            base_ascii: b"iidt00"
        },
        Pressure => {
            name: "Pressure",
            base_ascii: b"ipre00"
        },
        Lidar => {
            name: "Lidar",
            base_ascii: b"ilid00"
        },
        Audio => {
            name: "Audio",
            base_ascii: b"iear00"
        }
    }
}

// Outputs
define_indexed_cortical_enum! {
    OutputCorticalID {
        SpinningMotor => {
            name: "Spinning Motor",
            base_ascii: b"omot00"
        },
        ServoMotion => {
            name: "Servo (Delta Motion)",
            base_ascii: b"osmo00"
        },
        ServoPosition => {
            name: "Servo (Absolute Position)",
            base_ascii: b"ospo00"
        },
        MotionControl => {
            name: "Motion Control",
            base_ascii: b"omcl00"
        },
        Battery => {
            name: "Battery",
            base_ascii: b"pbat00"
        },
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

fn u8_to_hex_char_u8(byte: u8) -> (u8, u8) {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

    let high = HEX_CHARS[(byte >> 4) as usize];
    let low = HEX_CHARS[(byte & 0x0F) as usize];

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