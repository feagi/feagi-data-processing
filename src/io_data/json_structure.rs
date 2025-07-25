use serde_json;
use crate::io_processing::byte_structures::{FeagiByteStructureType, FeagiByteStructure, FeagiByteStructureCompatible};
use crate::error::{FeagiBytesError, FeagiDataProcessingError, IODataError};

#[derive(Clone)]
pub struct JsonStructure {
    json: serde_json::Value,
}

impl FeagiByteStructureCompatible for JsonStructure {
    fn get_type(&self) -> FeagiByteStructureType { Self::BYTE_STRUCTURE_TYPE }

    fn get_version(&self) -> u8 {Self:: BYTE_STRUCT_VERSION}

    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, FeagiDataProcessingError> {
        
        // doing this here instead of using the max_number_bytes_needed func so we can double-dip on data usage
        let json_string = self.json.to_string();
        let json_bytes = json_string.as_bytes();
        
        let num_bytes_needed: usize = FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + json_bytes.len();
        if slice.len() < num_bytes_needed {
            return Err(IODataError::InvalidInplaceOperation(format!("Not enough space given to store JSON data! Need {} bytes but given {}!", num_bytes_needed, slice.len())).into());
        }
        
        // Write the global header
        slice[0] = self.get_type() as u8;
        slice[1] = self.get_version();
        
        // Write the JSON data as UTF-8 bytes
        slice[FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES..FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + json_bytes.len()].copy_from_slice(json_bytes);
        
        let wasted_space = slice.len() - num_bytes_needed;
        Ok(wasted_space)
    }

    fn max_number_bytes_needed(&self) -> usize {
        // Global header (2 bytes) + JSON data as UTF-8 bytes
        // TODO this is pretty slow, any faster way to do this?
        FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + self.json.to_string().as_bytes().len()
    }

    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, FeagiDataProcessingError>
    where
        Self: Sized
    {
        FeagiByteStructure::verify_matching_structure_type_and_version(
            feagi_byte_structure,
            Self::BYTE_STRUCTURE_TYPE,
            Self::BYTE_STRUCT_VERSION
        )?;
        
        let bytes = feagi_byte_structure.borrow_data_as_slice();
        
        if bytes.len() < FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES {
            return Err(FeagiBytesError::UnableToDeserializeBytes("JSON byte structure too short to contain global header".into()).into());
        }
        
        // Extract JSON data (everything after the global header)
        let json_bytes = &bytes[FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES..];
        
        // Parse JSON string
        let json_value = match serde_json::from_slice(json_bytes) {
            Ok(value) => value,
            Err(e) => return Err(FeagiBytesError::UnableToDeserializeBytes(format!("Invalid JSON data: {}", e)).into()),
        };
        
        Ok(JsonStructure { json: json_value })
    }
}


impl JsonStructure {
    const BYTE_STRUCTURE_TYPE: FeagiByteStructureType = FeagiByteStructureType::JSON;
    const BYTE_STRUCT_VERSION: u8 = 1;
    
    pub fn from_json_string(string: String) -> Result<JsonStructure, FeagiDataProcessingError> {
        match serde_json::from_str(&string) {
            Ok(json_value) => Ok(JsonStructure { json: json_value }),
            Err(e) => Err(IODataError::InvalidParameters(
                format!("Failed to parse JSON string: {}", e)
            ).into()),
        }
    }
    
    pub fn from_json_value(value: serde_json::Value) -> JsonStructure {
        JsonStructure { json: value }
    }
    
    pub fn copy_as_json_string(&self) -> Result<String, FeagiDataProcessingError> {
        Ok(self.json.to_string())
    }
    
    pub fn borrow_json_value(&self) -> &serde_json::Value {
        &self.json
    }
}