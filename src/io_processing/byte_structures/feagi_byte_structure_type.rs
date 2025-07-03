use std::fmt::{Display, Formatter};
use crate::error::{FeagiBytesError, FeagiDataProcessingError};

/// Enumeration of all supported FEAGI byte structure format types.
/// 
/// Each variant corresponds to a specific serialization format and is used
/// as the first byte in the global header to identify the format type.
/// The discriminant values are explicitly specified to ensure stability
/// across different compiler versions and targets.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FeagiByteStructureType {
    /// JSON serialization format (human-readable text)
    JSON = 1,
    
    /// Multi-structure container format.
    /// 
    /// A container format that can hold multiple different FEAGI structures
    /// in a single serialized byte stream.
    MultiStructHolder = 9,
    
    /// Binary format for neuron categorical XYZP data.
    /// 
    /// Binary format specifically designed for neuron data
    /// with X, Y, Z coordinates and potential (P) values.
    NeuronCategoricalXYZP = 11
}

impl Display for FeagiByteStructureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            FeagiByteStructureType::JSON => "JSON",
            FeagiByteStructureType::MultiStructHolder => "MultiStructHolder",
            FeagiByteStructureType::NeuronCategoricalXYZP => "NeuronCategoricalXYZP",
        };
        write!(f, "{name}")
    }
}

impl FeagiByteStructureType {
    pub fn try_from(value: u8) -> Result<Self, FeagiDataProcessingError> {
        match value {
            1 => Ok(FeagiByteStructureType::JSON),
            9 => Ok(FeagiByteStructureType::MultiStructHolder),
            11 => Ok(FeagiByteStructureType::NeuronCategoricalXYZP),
            _ => Err(FeagiBytesError::UnableToDeserializeBytes(format!("Unknown FeagiByteStructure type {}", value)).into())
        }
    }
    pub fn try_get_type_from_bytes(bytes: &[u8]) -> Result<FeagiByteStructureType, FeagiDataProcessingError> {
        if bytes.len() < 1 {
            return Err(FeagiBytesError::UnableToDeserializeBytes("Cannot ascertain type of empty byte array!".into()).into())
        }
        FeagiByteStructureType::try_from(bytes[0])
    }

}
