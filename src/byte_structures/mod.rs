//! Byte structure serialization and deserialization framework for FEAGI.
//!
//! This module provides a comprehensive system for converting FEAGI data structures
//! to and from various byte formats. It implements a trait-based design that allows
//! for multiple serialization formats while maintaining type safety and performance.
//!
//! ## Supported Formats
//!
//! - **JSON (Type 1)**: Human-readable JSON serialization via [`serializers::b001_json`]
//! - **Multi-Struct Holder (Type 9)**: Container format for multiple structures via [`serializers::b009_multi_struct_holder`]
//! - **Neuron Categorical XYZP (Type 11)**: Efficient binary format for neuron data via [`serializers::b011_neuron_categorical_xyzp`]
//!
//! ## Global Header Format
//!
//! All serialized data begins with a 2-byte global header:
//! - Byte 0: Format type identifier ([`FeagiByteStructureType`])
//! - Byte 1: Format version number
//!
//! ## Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::byte_structures::{FeagiByteStructureType, GLOBAL_HEADER_SIZE};
//!
//! // Check format type from serialized data
//! let serialized_data = [11u8, 1u8, /* ... rest of data */];
//! match serialized_data[0] {
//!     11 => println!("Neuron categorical XYZP format"),
//!     9 => println!("Multi-struct holder format"),
//!     1 => println!("JSON format"),
//!     _ => println!("Unknown format"),
//! }
//! ```

pub mod feagi_byte_structure;

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use crate::byte_structures::feagi_byte_structure::FeagiByteStructure;
use crate::error::DataProcessingError;

/// Size in bytes of the global header that prefixes all FEAGI byte structures.
/// 
/// The global header consists of:
/// - 1 byte: Format type identifier (u8)
/// - 1 byte: Format version number (u8)
pub const GLOBAL_HEADER_SIZE: usize = 2; // TODO remove from here!


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
    pub fn try_from(value: u8) -> Result<Self, DataProcessingError> {
        match value {
            1 => Ok(FeagiByteStructureType::JSON),
            9 => Ok(FeagiByteStructureType::MultiStructHolder),
            11 => Ok(FeagiByteStructureType::NeuronCategoricalXYZP),
            _ => Err(DataProcessingError::InvalidByteStructure(format!("Unknown FeagiByteStructure type {}", value)))
        }
    }
    pub fn try_get_type_from_bytes(bytes: &[u8]) -> Result<FeagiByteStructureType, DataProcessingError> {
        if bytes.len() < 1 {
            return Err(DataProcessingError::InvalidByteStructure("Cannot ascertain type of empty byte array!".into()))
        }
        FeagiByteStructureType::try_from(bytes[0])
    }

}

pub trait FeagiByteStructureCompatible {

    fn get_type(&self) -> FeagiByteStructureType;
    fn get_version(&self) -> u8;
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, DataProcessingError>;
    fn max_number_bytes_needed(&self) -> usize;
    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, DataProcessingError> where Self: Sized;

    fn verify_slice_has_enough_space(&self, slice: &[u8]) -> Result<(), DataProcessingError> {
        if slice.len() < self.max_number_bytes_needed() {
            return Err(DataProcessingError::IncompatibleInplace(format!("Given slice is only {} bytes long when {} bytes of space are required!", slice.len(), self.max_number_bytes_needed())));
        }
        Ok(())
    }
    fn as_new_feagi_byte_structure(&self) -> Result<FeagiByteStructure, DataProcessingError> {
        let mut bytes: Vec<u8> = vec![0; self.max_number_bytes_needed()];
        _ = self.overwrite_feagi_byte_structure_slice(&mut bytes)?; // theoretically some bytes may be wasted
        FeagiByteStructure::create_from_bytes(bytes)
    }

}