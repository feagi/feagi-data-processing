use super::{FeagiByteStructureType, confirm_feagi_byte_structure_type};

const GLOBAL_HEADER_SIZE_IN_BYTES: u32 = 2;
use byteorder::{ByteOrder, LittleEndian};
pub fn from_multi_structure_holder_get_boundaries(bytes: &[u8], start_end_index: (usize, usize)) -> Result<Vec<(u32, u32)>, &'static str> {
    const INITIAL_HEADER_CONSTANT_SIZE: u32 = GLOBAL_HEADER_SIZE_IN_BYTES + 1; 
    const HEADER_2_ELEMENT_SIZE: u32 = 8;
    
    let length_of_bytes: u32 = bytes.len() as u32;
    
    let confirm = confirm_feagi_byte_structure_type(bytes, &start_end_index, FeagiByteStructureType::MultiStructHolder, 3);
    
    if confirm.is_err() {
        return Err(confirm.unwrap_err());
    }
    let number_contained_structs: u8 = bytes[2];
    let minimum_size_needed_for_second_header: u32 = INITIAL_HEADER_CONSTANT_SIZE + (number_contained_structs as u32 * HEADER_2_ELEMENT_SIZE);
    
    if length_of_bytes < minimum_size_needed_for_second_header {
        return Err("The byte array is smaller than specified by MultiStructHolder Sub Header 1!");
    }
    
    let mut output: Vec<(u32, u32)> = Vec::with_capacity(number_contained_structs as usize);
    let mut read_index: u32 = minimum_size_needed_for_second_header;
    
    for i in 0..number_contained_structs {
        let start = read_index as usize;
        let middle = read_index + 4;
        let end = start + 8;
        
        let first_bound: u32 = LittleEndian::read_u32(&bytes[start ..middle]);
        let end_bound: u32 = LittleEndian::read_u32(&bytes[middle ..end]);
        
        if first_bound > length_of_bytes || 
        
        
        output.push((
            LittleEndian::read_u32(&bytes[start ..middle]),
            LittleEndian::read_u32(&bytes[middle ..end]))
        );
        
    };
    
    if output.len() == 0 {
        return Ok(output);
    }
    
    
    
    
    
}











