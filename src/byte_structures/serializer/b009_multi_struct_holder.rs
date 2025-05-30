use crate::byte_structures::GLOBAL_HEADER_SIZE;
use crate::error::DataProcessingError;
use super::FeagiByteSerializer;
use byteorder::{ByteOrder, LittleEndian};


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
        let max_size = self.get_max_possible_size_when_serialized();
        let mut output = vec![0u8; max_size];

        // Write header
        output[0] = self.get_id();
        output[1] = self.get_version();
        output[2] = self.contained_serializers.len() as u8;
        
        let mut subheader_write_index: usize = 3;
        let sub_header_size_per_struct: usize = 8; // 4 bytes for position + 4 bytes for length
        let data_start_position: usize = 3 + (self.contained_serializers.len() * sub_header_size_per_struct);
        let mut current_data_position: usize = data_start_position;
        
        // Serialize each contained structure and write sub-headers
        for serializer in &self.contained_serializers {
            // Serialize the data for this structure
            let serialized_data = serializer.serialize_new()?;
            let data_length = serialized_data.len();
            
            // Write sub-header: position (4 bytes) + length (4 bytes)
            LittleEndian::write_u32(&mut output[subheader_write_index..subheader_write_index + 4], current_data_position as u32);
            LittleEndian::write_u32(&mut output[subheader_write_index + 4..subheader_write_index + 8], data_length as u32);
            
            // Copy the serialized data to the output
            output[current_data_position..current_data_position + data_length].copy_from_slice(&serialized_data);
            
            // Update positions for next iteration
            subheader_write_index += sub_header_size_per_struct;
            current_data_position += data_length;
        }

        Ok(output)
    }

    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {
        let num_bytes_needed = self.get_max_possible_size_when_serialized();
        if bytes_to_overwrite.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store MultiStructHolder! Need {} bytes but given {}!", num_bytes_needed, bytes_to_overwrite.len())));
        }

        // Write header
        bytes_to_overwrite[0] = self.get_id();
        bytes_to_overwrite[1] = self.get_version();
        bytes_to_overwrite[2] = self.contained_serializers.len() as u8;
        
        let mut subheader_write_index: usize = 3;
        let sub_header_size_per_struct: usize = 8; // 4 bytes for position + 4 bytes for length
        let data_start_position: usize = 3 + (self.contained_serializers.len() * sub_header_size_per_struct);
        let mut current_data_position: usize = data_start_position;
        
        // Serialize each contained structure and write sub-headers
        for serializer in &self.contained_serializers {
            // Serialize the data for this structure
            let serialized_data = serializer.serialize_new()?;
            let data_length = serialized_data.len();
            
            // Write sub-header: position (4 bytes) + length (4 bytes)
            LittleEndian::write_u32(&mut bytes_to_overwrite[subheader_write_index..subheader_write_index + 4], current_data_position as u32);
            LittleEndian::write_u32(&mut bytes_to_overwrite[subheader_write_index + 4..subheader_write_index + 8], data_length as u32);
            
            // Copy the serialized data to the output
            bytes_to_overwrite[current_data_position..current_data_position + data_length].copy_from_slice(&serialized_data);
            
            // Update positions for next iteration
            subheader_write_index += sub_header_size_per_struct;
            current_data_position += data_length;
        }
        
        // Return number of unused bytes
        Ok(bytes_to_overwrite.len() - current_data_position)
    }
}