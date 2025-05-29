use serde_json;
use crate::error::DataProcessingError;
use super::{FeagiByteDeserializer, FeagiByteStructureType, verify_header_of_full_structure_bytes};

pub struct JsonDeserializerV1<'internal_bytes> {
    data_slice: & 'internal_bytes[u8]
}

impl FeagiByteDeserializer for JsonDeserializerV1<'_> {
    fn get_id(&self) -> u8 {FeagiByteStructureType::JSON as u8}
    fn get_version(&self) -> u8 {1}
    
}


impl <'internal_bytes> JsonDeserializerV1<'internal_bytes> {
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<JsonDeserializerV1<'internal_bytes>, DataProcessingError> {
        verify_header_of_full_structure_bytes(data_slice, FeagiByteStructureType::JSON, 1)?;
        Ok(JsonDeserializerV1 { data_slice })
    }
    
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        // Json Struct has a 2 byte start header, cut this out
        serde_json::from_slice(&self.data_slice[2..])
    }
    
}
