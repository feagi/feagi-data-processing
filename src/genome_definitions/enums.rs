use std::fmt;
use crate::error::DataProcessingError;

const CORTICAL_ID_LENGTH: usize = 6;

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
pub enum CorticalID {
    Custom([u8; CORTICAL_ID_LENGTH]),
    Memory([u8; CORTICAL_ID_LENGTH]),
    Core(CoreCorticalID),
    Input(InputCorticalID),
    Output(OutputCorticalID),
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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



define_indexed_cortical_enum! {
    InputCorticalID {
        InfraredSensor => {
            name: "Infrared Sensor",
            base_ascii: b"iinf00"
        },
        VisionCenterGray => {
            name: "Center Vision Grayscale",
            base_ascii: b"ivcc00"
        },
        VisionTopLeftGray => {
            name: "Top Left Vision Grayscale",
            base_ascii: b"ivtl00"
        },
        
    }
}

define_indexed_cortical_enum! {
    OutputCorticalID {
        SpinningMotor => {
            name: "Spinning Motor",
            base_ascii: b"omot00"
        },
        ServoMotion => {
            name: "Servo (Delta Motion)",
            base_ascii: b"osmo00"
        }
    }
}





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