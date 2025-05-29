pub mod b001_json;
pub mod b009_multi_struct_holder;
pub mod b011_neuron_categorical_xyzp;

use crate::byte_data_functions::FeagiByteStructureType;
use crate::error::DataProcessingError;
use b001_json::*;
use crate::byte_data_functions::deserializers::b011_neuron_categorical_xyzp::NeuronCategoricalXYZPDeserializerV1;

enum Deserializer<'internal_bytes> {
    JsonV1(JsonDeserializerV1<'internal_bytes>),
    NeuronCategoricalXYZPV1(NeuronCategoricalXYZPDeserializerV1<'internal_bytes>),
}

trait FeagiByteDeserializer {
    fn get_id(&self) -> u8;
    fn get_version(&self) -> u8;
}

pub fn verify_header_of_full_structure_bytes(data: &[u8], expected_type: FeagiByteStructureType, expected_version: u8) -> Result<(), DataProcessingError> {
    if data.len() < 4 { // Header is 2 bytes, and we expected at LEAST 2 bytes of data
        return Err(DataProcessingError::InvalidByteStructure("Byte Data Structure is too short o hold a header and any meaningful data!".into()));
    }
    
    let expected_type= expected_type as u8;
    
    if data[0] != expected_type{
        return Err(DataProcessingError::InvalidByteStructure(format!("Incoming byte data has type ID {} when expected {}!", data[0], expected_type)));
    }
    if data[1] != expected_version {
        return Err(DataProcessingError::InvalidByteStructure(format!("Incoming byte data has version {} when expected {}!", data[0], expected_version)));
    }
    
    Ok(())
}

pub fn get_type_and_version_of_struct_from_bytes(data: &[u8]) -> Result<(FeagiByteStructureType, u8), DataProcessingError> {
    if data.len() < 4 { // Header is 2 bytes, and we expected at LEAST 2 bytes of data
        return Err(DataProcessingError::InvalidByteStructure("Byte Data Structure is too short to hold a header and any meaningful data!".into()));
    }
    let defined_type = data[0];
    match defined_type {
        1 => {
            Ok((FeagiByteStructureType::JSON, data[1]))
        }
        9 => {
            Ok((FeagiByteStructureType::MultiStructHolder, data[1]))
        }
        11 => {
            Ok((FeagiByteStructureType::NeuronCategoricalXYZP, data[1]))
        }
        _ => {
            Err(DataProcessingError::InvalidByteStructure(format!("Unknown byte Structure Type {}", defined_type)))
        }
        
    }
}

pub fn build_deserializer(bytes: &[u8]) -> Result<Deserializer, DataProcessingError> {
    
    let (structure_type, version) = get_type_and_version_of_struct_from_bytes(bytes)?;
    
    match structure_type {
        FeagiByteStructureType::JSON => {
            match version {
                1 => Ok(Deserializer::JsonV1(JsonDeserializerV1::from_data_slice(bytes)?)),
                _ => Err(DataProcessingError::InvalidByteStructure(format!("Unsupported version {} for JSON Deserializer!", bytes[0]))),
            }
        }
        
        
        /*
        FeagiByteStructureType::MultiStructHolder => { // FeagiByteStructureType::MultiStructHolder

        }
         */
        FeagiByteStructureType::MultiStructHolder => {
            match version {
                1 => Ok(Deserializer::NeuronCategoricalXYZPV1(NeuronCategoricalXYZPDeserializerV1::from_data_slice(bytes)?)),
                _ => Err(DataProcessingError::InvalidByteStructure(format!("Unsupported version {} for NeuronCategoricalXYZP Deserializer!", bytes[0]))),
            }
        }
        
        _ => {
            // This shouldn't be possible unless something is unimplemented
            Err(DataProcessingError::InternalError(format!("Missing deserializer definition for structure ID {}!", bytes[0])))
        }
    }
    
}