use crate::error::DataProcessingError;
use super::FeagiByteStructureType;

pub trait FeagiByteStructureCompatible {
    
    fn get_type(&self) -> FeagiByteStructureType;
    fn get_version(&self) -> u8;
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, DataProcessingError>;
    fn max_number_bytes_needed(&self) -> usize;
    
    fn verify_slice_has_enough_space(&self, slice: &[u8]) -> Result<(), DataProcessingError> {
        if slice.len() < self.max_number_bytes_needed() {
            return Err(DataProcessingError::IncompatibleInplace(format!("Given slice is only {} bytes long when {} bytes of space are required!", slice.len(), self.max_number_bytes_needed())));
        }
        Ok(())
    }
    fn as_new_feagi_byte_structure(&self) -> Result<FeagiByteStructure, DataProcessingError> {
        let mut bytes: Vec<u8> = vec![0; self.max_number_bytes_needed()];
        _ = self.overwrite_feagi_byte_structure_slice(&mut bytes)?; // theoretically some bytes may be wasted
        FeagiByteStructure::create_from_bytes(bytes)
    }
}


pub struct FeagiByteStructure {
    bytes: Vec<u8>,
}

impl FeagiByteStructure {
    const MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID: usize = 4;
    
    pub fn create_from_bytes(bytes: Vec<u8>) -> Result<FeagiByteStructure, DataProcessingError> {
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
    
    pub fn borrow_data_as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
    
    pub fn ensure_capacity_of_at_least(&mut self, size: usize) -> Result<(), DataProcessingError> {
        if size < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(DataProcessingError::InvalidInputBounds(format!("Cannot set capacity to less than minimum required capacity of {}!", Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID)));
        }
        
        if self.bytes.capacity() < size {
            self.bytes.reserve(self.bytes.capacity() - size);
        }
        Ok(())
    }
    
    pub fn shed_wasted_capacity(&mut self) {
        self.bytes.shrink_to_fit();
    }
    
    pub fn reset_write_index(&mut self) {
        self.bytes.truncate(0);
    }
    
    pub fn get_wasted_capacity_count(&self) -> usize {
        self.bytes.capacity() - self.bytes.len()
    }
    
    pub fn get_utilized_capacity_percentage(&self) -> f32 {
        (self.bytes.len() as f32 / self.bytes.capacity() as f32) * 100.0
    }
    
}