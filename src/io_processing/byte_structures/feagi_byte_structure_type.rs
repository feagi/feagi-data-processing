//! Type identification for FEAGI bytes structure formats.
//!
//! This module defines the `FeagiByteStructureType` enum, which provides standardized
//! identifiers for different serialization formats supported by the FEAGI system.
//! These type identifiers are embedded in the binary format headers to enable
//! format detection and proper deserialization.

use std::fmt::{Display, Formatter};
use crate::error::{FeagiBytesError, FeagiDataProcessingError};

/// Enumeration of all supported FEAGI bytes structure format types.
/// 
/// Each variant corresponds to a specific serialization format and is used
/// as the first bytes in the global header to identify the format type.
/// The discriminant values are explicitly specified to ensure stability
/// across different compiler versions and targets.
///
/// # Format Identification
///
/// The type identifier is always stored as the first bytes in any FEAGI bytes structure,
/// allowing immediate format detection without parsing the entire structure. This
/// enables efficient routing to appropriate deserialization logic.
///
/// # Stability Guarantee
///
/// The numeric values are explicitly assigned and must never change to maintain
/// backward compatibility. New formats should use new unused numeric identifiers.
///
/// # Usage in Headers
///
/// ```text
/// Byte 0: Format Type (this enum as u8)
/// Byte 1: Version number
/// Byte 2+: Format-specific data
/// ```
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FeagiByteStructureType {
    /// JSON serialization format (human-readable text)
    JSON = 1,
    
    /// Multi-structure container format.
    /// 
    /// A container format that can hold multiple different FEAGI structures
    /// in a single serialized bytes stream.
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
    /// Attempts to convert a numeric type identifier to a FeagiByteStructureType.
    ///
    /// This function maps the numeric identifiers stored in bytes structure headers
    /// back to their corresponding enum variants. Used during deserialization to
    /// determine the appropriate format handling logic.
    ///
    /// # Arguments
    /// * `value` - Numeric type identifier from a bytes structure header
    ///
    /// # Returns
    /// * `Ok(FeagiByteStructureType)` - Successfully identified format type
    /// * `Err(FeagiDataProcessingError)` - Unknown or unsupported type identifier
    ///
    /// # Supported Identifiers
    /// - `1` → JSON format
    /// - `9` → MultiStructHolder format  
    /// - `11` → NeuronCategoricalXYZP format
    pub fn try_from(value: u8) -> Result<Self, FeagiDataProcessingError> {
        match value {
            1 => Ok(FeagiByteStructureType::JSON),
            9 => Ok(FeagiByteStructureType::MultiStructHolder),
            11 => Ok(FeagiByteStructureType::NeuronCategoricalXYZP),
            _ => Err(FeagiBytesError::UnableToDeserializeBytes(format!("Unknown FeagiByteStructure type {}", value)).into())
        }
    }

    /// Extracts the format type from the first bytes of a bytes array.
    ///
    /// This convenience method reads the type identifier directly from raw bytes data,
    /// performing the necessary validation and conversion. Commonly used during the
    /// initial parsing of incoming bytes structures.
    ///
    /// # Arguments
    /// * `bytes` - Raw bytes array containing a FEAGI bytes structure
    ///
    /// # Returns
    /// * `Ok(FeagiByteStructureType)` - Successfully identified format type
    /// * `Err(FeagiDataProcessingError)` - If the bytes array is empty or contains an unknown type
    ///
    /// # Requirements
    /// The bytes array must contain at least one bytes. The first bytes is interpreted
    /// as the format type identifier according to the FEAGI bytes structure standard.
    pub fn try_get_type_from_bytes(bytes: &[u8]) -> Result<FeagiByteStructureType, FeagiDataProcessingError> {
        if bytes.len() < 1 {
            return Err(FeagiBytesError::UnableToDeserializeBytes("Cannot ascertain type of empty bytes array!".into()).into())
        }
        FeagiByteStructureType::try_from(bytes[0])
    }

}
