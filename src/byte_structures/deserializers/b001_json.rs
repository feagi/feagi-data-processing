//! JSON deserialization implementation for FEAGI data structures.
//!
//! This module provides deserialization capabilities for JSON-formatted FEAGI data.
//! It converts byte streams containing JSON data back into structured serde_json::Value
//! objects for further processing within the FEAGI system.
//!
//! ## Format Support
//!
//! - **Format Type**: 1 (JSON)
//! - **Supported Versions**: 1
//! - **Encoding**: UTF-8
//! - **Output Type**: `serde_json::Value`
//!
//! ## Usage Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::byte_structures::deserializers::{
//!     b001_json::JsonDeserializerV1, FeagiByteDeserializer
//! };
//!
//! // Deserialize JSON data from bytes
//! let json_bytes = [1u8, 1u8, b'{', b'"', b'k', b'e', b'y', b'"', b':', b'1', b'}'];
//! let deserializer = JsonDeserializerV1::from_data_slice(&json_bytes).unwrap();
//! let json_value = deserializer.to_json().unwrap();
//! ```

use serde_json;
use crate::error::DataProcessingError;
use super::{FeagiByteDeserializer, FeagiByteStructureType, verify_header_of_full_structure_bytes};

/// JSON deserializer for FEAGI byte structures (Format Type 1, Version 1).
///
/// This deserializer processes byte streams containing JSON data serialized by the
/// corresponding [`JsonSerializerV1`]. It validates the global header and converts
/// the UTF-8 encoded JSON payload back into structured data.
///
/// ## Format Structure
///
/// The deserializer expects byte data with this structure:
/// ```text
/// [Type ID: 1][Version: 1][JSON UTF-8 Data: Variable length]
/// ```
pub struct JsonDeserializerV1<'internal_bytes> {
    /// Reference to the complete byte slice containing the JSON structure.
    /// This includes the global header followed by UTF-8 encoded JSON data.
    data_slice: & 'internal_bytes[u8]
}

impl FeagiByteDeserializer for JsonDeserializerV1<'_> {
    /// Always returns the value corresponding to `FeagiByteStructureType::JSON` (1)
    fn get_id(&self) -> u8 {FeagiByteStructureType::JSON as u8}
    
    /// Always returns `1` for this version of the JSON deserializer
    fn get_version(&self) -> u8 {1}
}

impl <'internal_bytes> JsonDeserializerV1<'internal_bytes> {
    /// Creates a new JSON deserializer from a byte slice containing JSON data.
    ///
    /// This constructor validates the global header to ensure the data is compatible
    /// with JSON deserialization. It verifies both the format type and version
    /// before creating the deserializer instance.
    ///
    /// # Arguments
    /// 
    /// * `data_slice` - Byte slice containing the complete JSON structure with global header
    ///
    /// # Returns
    /// 
    /// - `Ok(JsonDeserializerV1)`: Successfully created deserializer
    /// - `Err(DataProcessingError)`: Header validation failed
    ///
    /// # Errors
    /// 
    /// - `InvalidByteStructure`: Data too short, wrong format type, or wrong version
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::byte_structures::deserializers::b001_json::JsonDeserializerV1;
    /// 
    /// // Valid JSON byte structure
    /// let json_data = [1u8, 1u8, b'{', b'"', b'k', b'e', b'y', b'"', b':', b'1', b'}'];
    /// let deserializer = JsonDeserializerV1::from_data_slice(&json_data).unwrap();
    /// 
    /// // Invalid format type will fail
    /// let invalid_data = [2u8, 1u8, b'{', b'}'];
    /// assert!(JsonDeserializerV1::from_data_slice(&invalid_data).is_err());
    /// ```
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<JsonDeserializerV1<'internal_bytes>, DataProcessingError> {
        verify_header_of_full_structure_bytes(data_slice, FeagiByteStructureType::JSON, 1)?;
        Ok(JsonDeserializerV1 { data_slice })
    }
    
    /// Deserializes the JSON data into a serde_json::Value.
    ///
    /// This method extracts the JSON payload from the byte structure (skipping the
    /// 2-byte global header) and parses it using serde_json. The result is a
    /// structured JSON value that can be further processed or converted to
    /// specific data types.
    ///
    /// # Returns
    /// 
    /// - `Ok(serde_json::Value)`: Successfully parsed JSON data
    /// - `Err(serde_json::Error)`: JSON parsing failed due to invalid syntax
    ///
    /// # Errors
    /// 
    /// Returns a `serde_json::Error` if the JSON payload is malformed, contains
    /// invalid UTF-8, or violates JSON syntax rules.
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::byte_structures::deserializers::b001_json::JsonDeserializerV1;
    /// use serde_json::json;
    /// 
    /// // Create deserializer from byte data
    /// let json_bytes = [1u8, 1u8, b'{', b'"', b'n', b'a', b'm', b'e', b'"', b':', 
    ///                   b'"', b'F', b'E', b'A', b'G', b'I', b'"', b'}'];
    /// let deserializer = JsonDeserializerV1::from_data_slice(&json_bytes).unwrap();
    /// 
    /// // Deserialize to JSON value
    /// let json_value = deserializer.to_json().unwrap();
    /// assert_eq!(json_value["name"], "FEAGI");
    /// 
    /// // Can also convert to specific types
    /// let name: String = json_value["name"].as_str().unwrap().to_string();
    /// ```
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        // Json Struct has a 2 byte start header, cut this out
        serde_json::from_slice(&self.data_slice[2..])
    }
}
