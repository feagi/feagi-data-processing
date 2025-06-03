use crate::error::DataProcessingError;
use super::FeagiByteStructureType;

/// Used when working with data coming directly from / to FEAGI
pub struct FeagiFullByteData {
    bytes: Vec<u8>,
}

impl FeagiFullByteData {
    const MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID: usize = 4;
    
    pub fn new(bytes: Vec<u8>) -> Result<FeagiFullByteData, DataProcessingError> {
        if bytes.len() < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte structure needs to be at least {} long to be considered valid. Given structure is only {} long!", Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID, bytes.len())));
        }
        _ = FeagiByteStructureType::try_from(bytes[0])?; // check if structure type is valid
        if bytes[1] == 0 {return Err(DataProcessingError::InvalidByteStructure("Byte structure cannot have version number 0!".into()));}
        // NOTE: Other checks go here
        
        Ok(Self { bytes })
    }
    
    pub fn try_get_structure_type(&self) -> Result<FeagiByteStructureType, DataProcessingError> {
        FeagiByteStructureType::try_from(self.bytes[0])
    }
    
    pub fn get_version(&self) -> u8 {
        self.bytes[1]
    }
    
    pub fn borrow_data_as_slice(&self) -> &[u8] {
        &self.bytes
    }
    
    pub fn borrow_data_as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}