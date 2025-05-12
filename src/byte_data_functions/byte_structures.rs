use super::{FeagiByteStructureType, confirm_feagi_byte_structure_is_as_expected};

const GLOBAL_HEADER_SIZE_IN_BYTES: u32 = 2;
const MULTISTRUCT_HEADER_SIZE_CONTAINED_NUMBER_OF_BYTE_STRUCTS: u32 = 1;
const MULTISTRUCT_SUBHEADER_SIZE_POSITIONING_OF_SUBSTRUCTURES: u32 = 8;


use byteorder::{ByteOrder, LittleEndian};

pub fn from_multi_structure_holder_get_sub_structures(bytes: &[u8]) -> Result<Vec<&[u8]>, &'static str> {
    let length_of_input_bytes: u32 = bytes.len() as u32;
    let check_if_multi_struct = confirm_feagi_byte_structure_is_as_expected(bytes, FeagiByteStructureType::MultiStructHolder);

    if check_if_multi_struct.is_err() {
        return Err(check_if_multi_struct.unwrap_err());
    }
    
    let number_contained_structs: u8 = bytes[2];
    let minimum_size_needed_for_metadata: u32 = GLOBAL_HEADER_SIZE_IN_BYTES +
        MULTISTRUCT_HEADER_SIZE_CONTAINED_NUMBER_OF_BYTE_STRUCTS +
        (MULTISTRUCT_SUBHEADER_SIZE_POSITIONING_OF_SUBSTRUCTURES * (number_contained_structs as u32));


    if length_of_input_bytes < minimum_size_needed_for_metadata {
        return Err("The input byte array is smaller than specified by MultiStructHolder Sub Header 1!");
    }

    let mut output: Vec<(&[u8])> = Vec::with_capacity(number_contained_structs as usize);
    let largest_possible_index = length_of_input_bytes as usize - 8; // Minus 2 * u32 size

    for reading_struct_index in 0..number_contained_structs {
        let starting_byte_u32_index = ((GLOBAL_HEADER_SIZE_IN_BYTES + MULTISTRUCT_HEADER_SIZE_CONTAINED_NUMBER_OF_BYTE_STRUCTS) +
            (reading_struct_index as u32 * MULTISTRUCT_SUBHEADER_SIZE_POSITIONING_OF_SUBSTRUCTURES)) as usize;
        let number_of_bytes_index = starting_byte_u32_index + 4;

        if starting_byte_u32_index >= largest_possible_index {
            return Err("Out of bound reading index location!");
        }

        let start = LittleEndian::read_u32(&bytes[starting_byte_u32_index .. starting_byte_u32_index + 4]) as usize;
        let end = start + LittleEndian::read_u32(&bytes[number_of_bytes_index .. number_of_bytes_index + 4]) as usize;
        
        if start > length_of_input_bytes as usize {
            return Err("Out of bound child structure start location!");
        }

        output.push(&bytes[start .. end]);
    };
    Ok(output)
}

pub fn from_multi_structure_holder_get_boundaries(bytes: &[u8]) -> Result<Vec<(u32, u32)>, &'static str> {

    
    let length_of_input_bytes: u32 = bytes.len() as u32;
    
    let check_if_multi_struct = confirm_feagi_byte_structure_is_as_expected(bytes, FeagiByteStructureType::MultiStructHolder);
    
    if check_if_multi_struct.is_err() {
        return Err(check_if_multi_struct.unwrap_err());
    }
    
    let number_contained_structs: u8 = bytes[2];
    let minimum_size_needed_for_metadata: u32 = GLOBAL_HEADER_SIZE_IN_BYTES +
        MULTISTRUCT_HEADER_SIZE_CONTAINED_NUMBER_OF_BYTE_STRUCTS +
        (MULTISTRUCT_SUBHEADER_SIZE_POSITIONING_OF_SUBSTRUCTURES * (number_contained_structs as u32));
    
    
    if length_of_input_bytes < minimum_size_needed_for_metadata {
        return Err("The input byte array is smaller than specified by MultiStructHolder Sub Header 1!");
    }
    
    let mut output: Vec<(u32, u32)> = Vec::with_capacity(number_contained_structs as usize);
    let largest_possible_index = length_of_input_bytes as usize - 8; // Minus 2 * u32 size
    
    for reading_struct_index in 0..number_contained_structs {
        let starting_byte_u32_index = ((GLOBAL_HEADER_SIZE_IN_BYTES + MULTISTRUCT_HEADER_SIZE_CONTAINED_NUMBER_OF_BYTE_STRUCTS) + 
            (reading_struct_index as u32 * MULTISTRUCT_SUBHEADER_SIZE_POSITIONING_OF_SUBSTRUCTURES)) as usize;
        let number_of_bytes_index = starting_byte_u32_index + 4;
        
        if starting_byte_u32_index >= largest_possible_index {
            return Err("Out of bound reading index location!");
        }
        
        let start_and_length: (u32, u32) = (LittleEndian::read_u32(&bytes[starting_byte_u32_index .. starting_byte_u32_index + 4]), LittleEndian::read_u32(&bytes[number_of_bytes_index .. number_of_bytes_index + 4]));
        if start_and_length.0 > length_of_input_bytes {
            return Err("Out of bound child structure start location!");
        }
        
        output.push(start_and_length);

    };
    Ok(output)
}
