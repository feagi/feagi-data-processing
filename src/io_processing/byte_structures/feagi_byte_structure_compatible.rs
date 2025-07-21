use crate::error::{FeagiBytesError, FeagiDataProcessingError};
use super::{FeagiByteStructure, FeagiByteStructureType};

pub trait FeagiByteStructureCompatible {

    fn get_type(&self) -> FeagiByteStructureType;
    fn get_version(&self) -> u8;
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, FeagiDataProcessingError>;
    fn max_number_bytes_needed(&self) -> usize;
    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, FeagiDataProcessingError> where Self: Sized;

    fn verify_slice_has_enough_space(&self, slice: &[u8]) -> Result<(), FeagiBytesError> {
        if slice.len() < self.max_number_bytes_needed() {
            return Err(FeagiBytesError::UnableToValidateBytes(format!("Given slice is only {} bytes long when {} bytes of space are required!", slice.len(), self.max_number_bytes_needed())));
        }
        Ok(())
    }
    fn as_new_feagi_byte_structure(&self) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        let mut bytes: Vec<u8> = vec![0; self.max_number_bytes_needed()];
        _ = self.overwrite_feagi_byte_structure_slice(&mut bytes)?; // theoretically some bytes may be wasted
        FeagiByteStructure::create_from_bytes(bytes)
    }

}