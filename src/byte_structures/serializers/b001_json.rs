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
        self.some_json.to_string().len() + GLOBAL_HEADER_SIZE
    }
    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError> {
        let mut bytes = Vec::with_capacity(self.get_max_possible_size_when_serialized());
        bytes.extend(self.generate_global_header().to_vec());
        bytes.extend(self.some_json.to_string().into_bytes());
        Ok(bytes)
    }
    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {
        let json_bytes = self.some_json.to_string().into_bytes();
        let num_bytes_needed = self.get_max_possible_size_when_serialized();
        if bytes_to_overwrite.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store JSON! Need {} bytes but given {}!", num_bytes_needed, bytes_to_overwrite.len())));
        }
        let num_extra_bytes = bytes_to_overwrite.len() - num_bytes_needed;
        bytes_to_overwrite[0] = self.get_id();
        bytes_to_overwrite[1] = self.get_version();
        bytes_to_overwrite[2..num_bytes_needed].copy_from_slice(&json_bytes);
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