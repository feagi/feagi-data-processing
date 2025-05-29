use serde_json;
use crate::byte_structures::GLOBAL_HEADER_SIZE;
use crate::error::DataProcessingError;
use super::FeagiByteSerializer;

pub struct JsonSerializerV1 {
    some_json: serde_json::Value,
}

impl FeagiByteSerializer for JsonSerializerV1 {
    fn get_id(&self) -> u8 { 1 }
    fn get_version(&self) -> u8 { 1 }
    fn get_max_possible_size_when_serialized(&self) -> usize {
        self.some_json.to_string().len()
    }
    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError> {
        Ok(self.some_json.to_string().into_bytes())
    }
    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {
        let bytes = self.some_json.to_string().into_bytes();
        let num_bytes_needed = bytes.len();
        if bytes_to_overwrite.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store JSON! Need {} bytes but given {}!", num_bytes_needed, bytes_to_overwrite.len())));
        }
        let num_extra_bytes = bytes_to_overwrite.len() - num_bytes_needed;
        bytes_to_overwrite[..num_bytes_needed].copy_from_slice(&bytes);
        Ok(num_extra_bytes)
    }
}

impl JsonSerializerV1 {
    pub fn from_json(json: serde_json::Value) -> Result<JsonSerializerV1, DataProcessingError> {
        Ok(JsonSerializerV1 { some_json: json })
    }
    
    pub fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.some_json
    }
}