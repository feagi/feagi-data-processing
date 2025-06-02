//! Deserialization implementations for FEAGI byte structures.
//!
//! This module provides a comprehensive framework for converting byte data back into
//! FEAGI data structures. It implements a trait-based system with automatic format
//! detection and version handling, enabling robust deserialization of data serialized
//! by the corresponding serializers.
//!
//! ## Architecture
//!
//! The deserialization system is built around:
//! - **Format Detection**: Automatic identification of byte structure types from headers
//! - **Version Support**: Graceful handling of different format versions  
//! - **Trait System**: Common interface via [`FeagiByteDeserializer`] trait
//! - **Factory Pattern**: Automatic deserializer creation via [`build_deserializer`]
//!
//! ## Supported Formats
//!
//! - **JSON (Type 1)**: Human-readable JSON deserialization via [`b001_json`]
//! - **Multi-Struct Holder (Type 9)**: Container format deserialization via [`b009_multi_struct_holder`]  
//! - **Neuron Categorical XYZP (Type 11)**: Efficient binary neuron data via [`b011_neuron_categorical_xyzp`]
//!
//! ## Error Handling
//!
//! The deserialization system provides comprehensive error handling for:
//! - Invalid or corrupted headers
//! - Unsupported format versions
//! - Insufficient data length
//! - Format-specific validation errors
//!
//! ## Header Validation
//!
//! All deserializers validate the global header structure and provide utilities
//! for verifying format types and versions before attempting deserialization.

/// JSON deserialization implementation (format type 1).
/// 
/// Provides deserialization of human-readable JSON data back into serde_json::Value
/// objects for further processing.
pub mod b001_json;

/// Multi-structure container deserialization implementation (format type 9).
/// 
/// Handles deserialization of container formats that hold multiple different
/// FEAGI structures, enabling extraction of individual contained structures.
pub mod b009_multi_struct_holder;

/// Neuron categorical XYZP binary deserialization implementation (format type 11).
/// 
/// Highly optimized deserialization of binary neuron data with X, Y, Z coordinates
/// and potential values, organized by cortical areas.
pub mod b011_neuron_categorical_xyzp;

use crate::byte_structures::FeagiByteStructureType;
use crate::error::DataProcessingError;
use b001_json::*;
use b009_multi_struct_holder::*;
use b011_neuron_categorical_xyzp::*;

/// Enumeration of all available deserializer types.
///
/// This enum provides a type-safe way to handle different deserializer implementations
/// while maintaining a common interface. Each variant corresponds to a specific
/// format type and version combination.
///
/// The lifetime parameter `'internal_bytes` represents the lifetime of the byte data
/// being deserialized, ensuring memory safety when working with borrowed data.
pub enum Deserializer<'internal_bytes> {
    /// JSON deserializer version 1 for human-readable JSON data.
    JsonV1(JsonDeserializerV1<'internal_bytes>),
    
    /// Neuron categorical XYZP deserializer version 1 for binary neural data.
    NeuronCategoricalXYZPV1(NeuronCategoricalXYZPDeserializerV1<'internal_bytes>),
    
    /// Multi-structure holder deserializer version 1 for container formats.
    MultiStructHolderV1(MultiStructHolderDeserializerV1<'internal_bytes>)
}

/// Common trait for all FEAGI byte structure deserializers.
///
/// This trait provides a unified interface for all deserializer implementations,
/// enabling generic handling of different format types while maintaining
/// type safety and performance.
///
/// ## Implementation Requirements
///
/// All deserializers must provide format identification methods to enable
/// proper routing and validation during the deserialization process.
pub trait FeagiByteDeserializer {
    /// Returns the format identifier for this deserializer.
    ///
    /// This should match the format ID used in the global header of the
    /// corresponding serializer.
    ///
    /// # Returns
    ///
    /// Format type identifier (1-255)
    fn get_id(&self) -> u8;
    
    /// Returns the version number for this deserializer implementation.
    ///
    /// This should match the version used in the global header of the
    /// corresponding serializer.
    ///
    /// # Returns
    ///
    /// Version number (typically starting from 1)
    fn get_version(&self) -> u8;
}

/// Verifies the header of a complete byte structure against expected values.
///
/// This utility function validates that byte data contains the expected format type
/// and version before attempting deserialization. It provides early validation
/// to prevent processing of incompatible or corrupted data.
///
/// # Arguments
///
/// * `data` - Byte slice containing the complete structure with header
/// * `expected_type` - Expected format type from [`FeagiByteStructureType`]
/// * `expected_version` - Expected version number
///
/// # Returns
///
/// - `Ok(())`: Header is valid and matches expectations
/// - `Err(DataProcessingError)`: Header validation failed
///
/// # Errors
///
/// - `InvalidByteStructure`: Data too short, wrong type, or wrong version
pub fn verify_header_of_full_structure_bytes(data: &[u8], expected_type: FeagiByteStructureType, expected_version: u8) -> Result<(), DataProcessingError> {
    if data.len() < 4 { // Header is 2 bytes, and we expect at LEAST 2 bytes of data
        return Err(DataProcessingError::InvalidByteStructure("Byte Data Structure is too short to hold a header and any meaningful data!".into()));
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

/// Extracts format type and version information from byte structure headers.
///
/// This function reads the global header from byte data and returns the format
/// type and version information. It's used for automatic format detection
/// before creating appropriate deserializers.
///
/// # Arguments
///
/// * `data` - Byte slice containing the structure with global header
///
/// # Returns
///
/// - `Ok((FeagiByteStructureType, u8))`: Successfully parsed format type and version
/// - `Err(DataProcessingError)`: Header parsing failed
///
/// # Errors
///
/// - `InvalidByteStructure`: Data too short or unknown format type
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

/// Factory function to automatically create the appropriate deserializer from byte data.
///
/// This function examines the global header of the provided byte data and creates
/// the corresponding deserializer implementation. It handles format detection and
/// version validation automatically, providing a convenient entry point for
/// deserialization operations.
///
/// # Arguments
///
/// * `bytes` - Byte slice containing the complete serialized structure with header
///
/// # Returns
///
/// - `Ok(Deserializer)`: Successfully created deserializer for the detected format
/// - `Err(DataProcessingError)`: Format detection failed or unsupported format/version
///
/// # Errors
///
/// - `InvalidByteStructure`: Invalid header, unknown format, or unsupported version
/// - `InternalError`: Missing deserializer implementation (should not occur)
///
pub fn build_deserializer(bytes: &[u8]) -> Result<Deserializer, DataProcessingError> {
    
    let (structure_type, version) = get_type_and_version_of_struct_from_bytes(bytes)?;
    
    match structure_type {
        FeagiByteStructureType::JSON => {
            match version {
                1 => Ok(Deserializer::JsonV1(JsonDeserializerV1::from_data_slice(bytes)?)),
                _ => Err(DataProcessingError::InvalidByteStructure(format!("Unsupported version {} for JSON Deserializer!", bytes[0]))),
            }
        }
        FeagiByteStructureType::MultiStructHolder => {
            match version {
                1 => Ok(Deserializer::MultiStructHolderV1(MultiStructHolderDeserializerV1::from_data_slice(bytes)?)),
                _ => Err(DataProcessingError::InvalidByteStructure(format!("Unsupported version {} for MultiStructHolder Deserializer!", bytes[0]))),
            }
        }
        FeagiByteStructureType::NeuronCategoricalXYZP => {
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