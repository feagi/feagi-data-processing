use crate::error::DataProcessingError;
use super::{FeagiByteDeserializer, FeagiByteStructureType, verify_header_of_full_structure_bytes};

pub struct MultiStructHolderV1<'internal_bytes> {
    data_slice: &'internal_bytes [u8],
}

impl FeagiByteDeserializer for MultiStructHolderV1<'_> {
    fn get_id(&self) -> u8 {FeagiByteStructureType::MultiStructHolder as u8}
    fn get_version(&self) -> u8 {1}
}

impl<'internal_bytes> MultiStructHolderV1<'internal_bytes> {
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<MultiStructHolderV1<'internal_bytes>, DataProcessingError> {
        verify_header_of_full_structure_bytes(data_slice, FeagiByteStructureType::MultiStructHolder, 1)?;
        Ok(MultiStructHolderV1 { data_slice })
    }
    
    
    
}
