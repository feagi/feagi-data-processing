mod byte_structures;

use std::cmp::PartialEq;


#[derive(Debug, PartialEq)]
pub enum FeagiByteStructureType {
    MultiStructHolder = 9,
    NeuronPotentialCategoricalXYZ = 11
}

pub fn infer_feagi_byte_structure_type(bytes: &[u8]) -> Result<FeagiByteStructureType, &'static str> {
    if bytes.len() == 0 {
        return Err("Empty byte array!");
    }
    match bytes[0]
    {
        0 => Ok(FeagiByteStructureType::MultiStructHolder),
        11 => Ok(FeagiByteStructureType::NeuronPotentialCategoricalXYZ),
        _ => Err("Unknown byte array header!")
    }
}

pub fn infer_feagi_byte_structure_type_region(bytes: &[u8], start_end_index: &(usize, usize)) -> Result<FeagiByteStructureType, &'static str> {
    if start_end_index.0 >= start_end_index.1 {
        return Err("The start index cannot be after the end index!");
    }
    if start_end_index.1 > bytes.len() {
        return Err("The end index cannot be out of bounds of the byte array!");
    }
    infer_feagi_byte_structure_type(&bytes[start_end_index.0 .. start_end_index.1])
}

/// Generic function for checking if the basics of a byte structure are sound
fn confirm_feagi_byte_structure_type(bytes: &[u8], start_end_index: &(usize, usize), inferred_types: FeagiByteStructureType, minimum_length: usize) -> Result<(), &'static str> {
    if bytes.len() < minimum_length {
        return Err("The byte array cannot be shorter than the minimum length!");
    }
    let calculated_type = infer_feagi_byte_structure_type_region(bytes, start_end_index);
    if calculated_type.is_err() {
        return Err(&calculated_type.unwrap_err());
    }
    if calculated_type.unwrap() == inferred_types {
        return Ok(());
    }
    Err("Inferred types are not the same!")
}
