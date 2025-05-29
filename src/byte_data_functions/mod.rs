mod byte_structures;
mod deserializers;

use std::cmp::PartialEq;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FeagiByteStructureType {
    JSON = 1,
    MultiStructHolder = 9,
    NeuronCategoricalXYZP = 11
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

pub fn confirm_feagi_byte_structure_is_as_expected(bytes: &[u8], confirming_type: FeagiByteStructureType) -> Result<bool, &'static str> {
    let calculated_type = infer_feagi_byte_structure_type(bytes);
    if calculated_type.is_err() {
        return Err(calculated_type.unwrap_err());
    }
    Ok(calculated_type.unwrap() == confirming_type)
}

