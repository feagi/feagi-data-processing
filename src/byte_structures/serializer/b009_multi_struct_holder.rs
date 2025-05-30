use crate::byte_structures::GLOBAL_HEADER_SIZE;
use crate::error::DataProcessingError;
use super::FeagiByteSerializer;

pub struct MultiStructSerializerV1 {
    contained_serializers: Vec<Box<dyn FeagiByteSerializer>>,
}

impl FeagiByteSerializer for MultiStructSerializerV1 {
    fn get_id(&self) -> u8 { 9 }
    fn get_version(&self) -> u8 { 1 }
    fn get_max_possible_size_when_serialized(&self) -> usize {
        let mut output = 3; // 1 byte for the number contained struct header + global headers
        for serializer in &self.contained_serializers {
            output += serializer.get_max_possible_size_when_serialized() + 8;
        }
        output
    }

    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError> {
        let mut output = Vec::with_capacity(self.get_max_possible_size_when_serialized());

        output[0] = self.get_id();
        output[1] = self.get_version();
        
        output[2] = self.contained_serializers.len() as u8;
        
        let mut subheader_write_index: usize = 3;
        
        for serializer in &self.contained_serializers {
            
        }
        
        
        Ok(output)
    }

    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {
        todo!()
    }
}