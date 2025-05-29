use serde_json;
use crate::error::DataProcessingError;
use super::FeagiByteDeserializer;
use crate::byte_data_functions::FeagiByteStructureType;

pub struct JsonDeserializerV1<'internal_bytes> {
    data_slice: & 'internal_bytes[u8]
}

impl FeagiByteDeserializer for JsonDeserializerV1<'_> {
    fn get_id(&self) -> u8 {FeagiByteStructureType::JSON as u8}
    fn get_version(&self) -> u8 {1}
    
}


impl <'internal_bytes> JsonDeserializerV1<'internal_bytes> {
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<JsonDeserializerV1<'internal_bytes>, DataProcessingError> {
        // Validate header manually
        if data_slice.len() < 2 {
            return Err(DataProcessingError::InvalidByteStructure("Byte Data Structure is too short to hold even a header!".into()));
        }
        if data_slice[0] != 1 { // JSON ID
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte Data Structure has an ID of {} when expected 1!", data_slice[0])));
        }
        if data_slice[1] != 1 { // Version 1
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte Data Structure has an version of {} when expected 1!", data_slice[1])));
        }
        
        Ok(JsonDeserializerV1 { data_slice })
    }
    
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        // Json Struct has a 2 byte start header, cut this out
        serde_json::from_slice(&self.data_slice[2..])
    }
    
}
