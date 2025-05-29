use byteorder::{ByteOrder, LittleEndian};
use crate::error::DataProcessingError;
use super::{FeagiByteDeserializer, FeagiByteStructureType, verify_header_of_full_structure_bytes, Deserializer, build_deserializer};

pub struct MultiStructHolderDeserializerV1<'internal_bytes> {
    data_slice: &'internal_bytes [u8],
}

impl FeagiByteDeserializer for MultiStructHolderDeserializerV1<'_> {
    fn get_id(&self) -> u8 {FeagiByteStructureType::MultiStructHolder as u8}
    fn get_version(&self) -> u8 {1}
}

impl<'internal_bytes> MultiStructHolderDeserializerV1<'internal_bytes> {
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<MultiStructHolderDeserializerV1<'internal_bytes>, DataProcessingError> {
        verify_header_of_full_structure_bytes(data_slice, FeagiByteStructureType::MultiStructHolder, 1)?;
        Ok(MultiStructHolderDeserializerV1 { data_slice })
    }
    
    pub fn to_multiple_structs(&self) -> Result<Vec<Deserializer> , DataProcessingError> {
        const SUB_HEADER_SIZE_PER_STRUCT: usize = 8;
        let number_contained_structs: usize = self.data_slice[2] as usize; // This header element is the count as a u8
        let minimum_number_of_bytes_for_headers: usize = crate::byte_data_functions::GLOBAL_HEADER_SIZE + 1 + (number_contained_structs * SUB_HEADER_SIZE_PER_STRUCT);
        
        if self.data_slice.len() < minimum_number_of_bytes_for_headers {
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte structure for MultiStructHolderV1 needs a length of {} to fit just the cortical details header, but is a length of {}",
                                                                         minimum_number_of_bytes_for_headers, self.data_slice.len())));
        }
        
        let mut output: Vec<Deserializer> = Vec::with_capacity(number_contained_structs);
        let mut reading_index: usize = 3;
        
        for i in 0..number_contained_structs {
            let data_start_reading: usize = LittleEndian::read_u32(&self.data_slice[reading_index..reading_index+4]) as usize;
            let number_bytes_to_read: usize = LittleEndian::read_u32(&self.data_slice[reading_index+4..reading_index+8]) as usize;

            if self.data_slice.len() < minimum_number_of_bytes_for_headers + data_start_reading + number_bytes_to_read {
                return Err(DataProcessingError::InvalidByteStructure("Byte structure for MultiStructHolderV1 is too short to fit the data the header says it contains!".into()));
            }
            output.push(build_deserializer(&self.data_slice[data_start_reading..data_start_reading+number_bytes_to_read])?);
        }
        
        Ok(output)
    }
    
    
}
