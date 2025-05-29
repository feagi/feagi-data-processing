pub mod b001_json;

use crate::error::*;
use crate::error::DataProcessingError::InvalidByteStructure;

#[repr(u8)]
pub enum FeagiByteStructureType {
    JSON = 1,
    MultiStructHolder = 9,
    NeuronCategoricalXYZP = 11
}

pub trait FeagiByteDeserializer {
    fn get_id() -> u8;
    fn get_version() -> u8;

    fn validate_header(data: &[u8]) -> Result<(), DataProcessingError> {
        if data.len() < 2 {
            return Err(InvalidByteStructure("Byte Data Structure is too short to hold even a header!".into()));
        }
        if data[0] != Self::get_id() {
            return Err(InvalidByteStructure(format!("Byte Data Structure has an ID of {} when expected {}!", data[0], Self::get_id())));
        }
        if data[1] != Self::get_version() {
            return Err(InvalidByteStructure(format!("Byte Data Structure has an version of {} when expected {}!", data[1], Self::get_version())));
        }
        Ok(())
    }
}

pub fn deserialize<'byte_arr>(bytes: &'byte_arr Vec<u8>) -> Result<Box<dyn FeagiByteDeserializer>, DataProcessingError> {
    if bytes.len() < 2 {
        return Err(InvalidByteStructure("Byte Data Structure is too short to hold even a header!".into()));
    }
    
    match bytes[0]{
        FeagiByteStructureType::JSON => {
            b001_json::JsonDeserializerV1::from_data_slice(&bytes)
        }
        FeagiByteStructureType::MultiStructHolder => {
            
        }
        FeagiByteStructureType::NeuronCategoricalXYZP => {
            
        }
        _ => {
            return Err(InvalidByteStructure(format!("Unknown byte Structure type {}!", bytes[0])));
        }
    }
}