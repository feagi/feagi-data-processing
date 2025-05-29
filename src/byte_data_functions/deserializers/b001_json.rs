use serde_json;
use crate::error::DataProcessingError;
use super::FeagiByteDeserializer;

pub struct JsonDeserializerV1<'internal_bytes> {
    data_slice: & 'internal_bytes[u8]
}

impl <'internal_bytes> JsonDeserializerV1<'internal_bytes> {
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<JsonDeserializerV1<'internal_bytes>, DataProcessingError> {
        JsonDeserializerV1::validate_header(data_slice)?;
        Ok(JsonDeserializerV1 { data_slice })
    }
    
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        // Json Struct has a 2 byte start header, cut this out
        serde_json::from_slice(&self.data_slice[2..])
    }
    
}
impl FeagiByteDeserializer for JsonDeserializerV1<'_> {
    fn get_id() -> u8 {1}
    fn get_version() -> u8 {1}

}
