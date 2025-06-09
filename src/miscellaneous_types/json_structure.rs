use serde_json;
use crate::byte_structures::{FeagiByteStructureCompatible, FeagiByteStructureType, GLOBAL_HEADER_SIZE};
use crate::byte_structures::feagi_byte_structure::{FeagiByteStructure, verify_matching_structure_type_and_version};
use crate::error::DataProcessingError;

pub struct JsonStructure {
    json: serde_json::Value,
}

impl FeagiByteStructureCompatible for JsonStructure {
    fn get_type(&self) -> FeagiByteStructureType { Self::BYTE_STRUCTURE_TYPE }

    fn get_version(&self) -> u8 {Self:: BYTE_STRUCT_VERSION}

    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, DataProcessingError> {
        
        // doing this here instead of using the max_number_bytes_needed func so we can double-dip on data usage
        let json_string = self.json.to_string();
        let json_bytes = json_string.as_bytes();
        
        let num_bytes_needed: usize = json_bytes.len();
        if slice.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store JSON data! Need {} bytes but given {}!", num_bytes_needed, slice.len())));
        }
        
        // Write the global header
        slice[0] = self.get_type() as u8;
        slice[1] = self.get_version();
        
        // Write the JSON data as UTF-8 bytes
        slice[GLOBAL_HEADER_SIZE..GLOBAL_HEADER_SIZE + json_bytes.len()].copy_from_slice(json_bytes);
        
        let wasted_space = slice.len() - num_bytes_needed;
        Ok(wasted_space)
    }

    fn max_number_bytes_needed(&self) -> usize {
        // Global header (2 bytes) + JSON data as UTF-8 bytes
        // TODO this is pretty slow, any faster way to do this?
        GLOBAL_HEADER_SIZE + self.json.to_string().as_bytes().len()
    }

    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, DataProcessingError>
    where
        Self: Sized
    {
        verify_matching_structure_type_and_version(
            feagi_byte_structure,
            Self::BYTE_STRUCTURE_TYPE,
            Self::BYTE_STRUCT_VERSION
        )?;
        
        let bytes = feagi_byte_structure.borrow_data_as_slice();
        
        if bytes.len() < GLOBAL_HEADER_SIZE {
            return Err(DataProcessingError::InvalidByteStructure(
                "JSON byte structure too short to contain global header".to_string()
            ));
        }
        
        // Extract JSON data (everything after the global header)
        let json_bytes = &bytes[GLOBAL_HEADER_SIZE..];
        
        // Parse JSON string
        let json_value = match serde_json::from_slice(json_bytes) {
            Ok(value) => value,
            Err(e) => return Err(DataProcessingError::InvalidByteStructure(
                format!("Invalid JSON data: {}", e)
            )),
        };
        
        Ok(JsonStructure { json: json_value })
    }
}


impl JsonStructure {
    const BYTE_STRUCTURE_TYPE: FeagiByteStructureType = FeagiByteStructureType::JSON;
    const BYTE_STRUCT_VERSION: u8 = 1;
    
    pub fn from_json_string(string: String) -> Result<JsonStructure, DataProcessingError> {
        match serde_json::from_str(&string) {
            Ok(json_value) => Ok(JsonStructure { json: json_value }),
            Err(e) => Err(DataProcessingError::InvalidByteStructure(
                format!("Failed to parse JSON string: {}", e)
            )),
        }
    }
    
    pub fn from_json_value(value: serde_json::Value) -> JsonStructure {
        JsonStructure { json: value }
    }
    
    pub fn copy_as_json_string(&self) -> Result<String, DataProcessingError> {
        Ok(self.json.to_string())
    }
    
    pub fn borrow_json_value(&self) -> &serde_json::Value {
        &self.json
    }
}