//! JSON data structure with FEAGI byte structure compatibility.
//!
//! This module provides the `JsonStructure` type, which wraps JSON data and implements
//! the `FeagiByteStructureCompatible` trait for seamless integration with the FEAGI
//! byte structure serialization system. This allows JSON data to be stored, transmitted,
//! and processed alongside other FEAGI data types.

use serde_json;
use crate::io_processing::byte_structures::{FeagiByteStructureType, FeagiByteStructure, FeagiByteStructureCompatible};
use crate::error::{FeagiBytesError, FeagiDataProcessingError, IODataError};

/// JSON data structure compatible with FEAGI byte structure serialization.
///
/// `JsonStructure` wraps a `serde_json::Value` and provides FEAGI byte structure
/// compatibility, allowing JSON data to be serialized, transmitted, and stored
/// using the same infrastructure as other FEAGI data types.
///
/// # Features
///
/// - **Byte Structure Compatibility**: Implements `FeagiByteStructureCompatible` for
///   seamless integration with FEAGI serialization systems
/// - **JSON Validation**: Ensures JSON data is well-formed during construction
/// - **UTF-8 Encoding**: Uses UTF-8 encoding for string representation in byte structures
/// - **Flexible Input**: Accepts JSON from strings or `serde_json::Value` objects
///
/// # Serialization Format
///
/// The byte structure format consists of:
/// ```text
/// [Type=1 (1 byte)][Version=1 (1 byte)][JSON UTF-8 bytes...]
/// ```
///
/// # Usage Examples
///
/// ## Creating from JSON String
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::JsonStructure;
///
/// let json_str = r#"{"temperature": 23.5, "humidity": 60}"#;
/// let json_struct = JsonStructure::from_json_string(json_str.to_string()).unwrap();
/// ```
///
/// ## Creating from serde_json::Value
/// ```rust
/// use serde_json::json;
/// use feagi_core_data_structures_and_processing::io_data::JsonStructure;
///
/// let json_value = json!({"status": "active", "count": 42});
/// let json_struct = JsonStructure::from_json_value(json_value);
/// ```
///
/// # Integration with FEAGI Systems
///
/// JsonStructure can be used with:
/// - **Byte Structure Serialization**: For network transmission and storage
/// - **IOTypeData**: As configuration data for cortical areas
/// - **Stream Processing**: For metadata and control messages
/// - **Multi-Structure Containers**: Combined with other FEAGI data types
#[derive(Clone)]
pub struct JsonStructure {
    json: serde_json::Value,
}

impl FeagiByteStructureCompatible for JsonStructure {
    fn get_type(&self) -> FeagiByteStructureType { Self::BYTE_STRUCTURE_TYPE }

    fn get_version(&self) -> u8 {Self:: BYTE_STRUCT_VERSION}

    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, FeagiDataProcessingError> {
        
        // doing this here instead of using the max_number_bytes_needed func so we can double-dip on data usage
        let json_string = self.json.to_string();
        let json_bytes = json_string.as_bytes();
        
        let num_bytes_needed: usize = FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + json_bytes.len();
        if slice.len() < num_bytes_needed {
            return Err(IODataError::InvalidInplaceOperation(format!("Not enough space given to store JSON data! Need {} bytes but given {}!", num_bytes_needed, slice.len())).into());
        }
        
        // Write the global header
        slice[0] = self.get_type() as u8;
        slice[1] = self.get_version();
        
        // Write the JSON data as UTF-8 bytes
        slice[FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES..FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + json_bytes.len()].copy_from_slice(json_bytes);
        
        let wasted_space = slice.len() - num_bytes_needed;
        Ok(wasted_space)
    }

    fn max_number_bytes_needed(&self) -> usize {
        // Global header (2 bytes) + JSON data as UTF-8 bytes
        // TODO this is pretty slow, any faster way to do this?
        FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + self.json.to_string().as_bytes().len()
    }

    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, FeagiDataProcessingError>
    where
        Self: Sized
    {
        FeagiByteStructure::verify_matching_structure_type_and_version(
            feagi_byte_structure,
            Self::BYTE_STRUCTURE_TYPE,
            Self::BYTE_STRUCT_VERSION
        )?;
        
        let bytes = feagi_byte_structure.borrow_data_as_slice();
        
        if bytes.len() < FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES {
            return Err(FeagiBytesError::UnableToDeserializeBytes("JSON byte structure too short to contain global header".into()).into());
        }
        
        // Extract JSON data (everything after the global header)
        let json_bytes = &bytes[FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES..];
        
        // Parse JSON string
        let json_value = match serde_json::from_slice(json_bytes) {
            Ok(value) => value,
            Err(e) => return Err(FeagiBytesError::UnableToDeserializeBytes(format!("Invalid JSON data: {}", e)).into()),
        };
        
        Ok(JsonStructure { json: json_value })
    }
}

impl JsonStructure {
    /// The FEAGI byte structure type identifier for JSON data.
    const BYTE_STRUCTURE_TYPE: FeagiByteStructureType = FeagiByteStructureType::JSON;
    
    /// The current version of the JSON byte structure format.
    const BYTE_STRUCT_VERSION: u8 = 1;
    
    /// Creates a JsonStructure from a JSON string with validation.
    ///
    /// Parses the provided JSON string and validates that it represents well-formed JSON.
    /// The parsed JSON is stored internally as a `serde_json::Value` for efficient access.
    ///
    /// # Arguments
    /// * `string` - A JSON string to parse and store
    ///
    /// # Returns
    /// * `Ok(JsonStructure)` - Successfully parsed and validated JSON
    /// * `Err(FeagiDataProcessingError)` - If the string is not valid JSON
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::JsonStructure;
    ///
    /// let valid_json = JsonStructure::from_json_string(r#"{"key": "value"}"#.to_string()).unwrap();
    /// let invalid_json = JsonStructure::from_json_string("not json".to_string());
    /// assert!(invalid_json.is_err());
    /// ```
    pub fn from_json_string(string: String) -> Result<JsonStructure, FeagiDataProcessingError> {
        match serde_json::from_str(&string) {
            Ok(json_value) => Ok(JsonStructure { json: json_value }),
            Err(e) => Err(IODataError::InvalidParameters(
                format!("Failed to parse JSON string: {}", e)
            ).into()),
        }
    }
    
    /// Creates a JsonStructure from a pre-existing serde_json::Value.
    ///
    /// This method wraps an already-parsed JSON value without additional validation,
    /// since `serde_json::Value` is guaranteed to represent valid JSON structure.
    ///
    /// # Arguments
    /// * `value` - A validated `serde_json::Value` to wrap
    ///
    /// # Returns
    /// A new `JsonStructure` containing the provided JSON value
    ///
    /// # Example
    /// ```rust
    /// use serde_json::json;
    /// use feagi_core_data_structures_and_processing::io_data::JsonStructure;
    ///
    /// let json_value = json!({"sensor": "temperature", "value": 23.5});
    /// let json_struct = JsonStructure::from_json_value(json_value);
    /// ```
    pub fn from_json_value(value: serde_json::Value) -> JsonStructure {
        JsonStructure { json: value }
    }
    
    /// Returns a JSON string representation of the stored data.
    ///
    /// Converts the internal `serde_json::Value` back to a formatted JSON string.
    /// The resulting string is guaranteed to be valid JSON that can be parsed
    /// by any standard JSON parser.
    ///
    /// # Returns
    /// * `Ok(String)` - JSON string representation of the data
    /// * `Err(FeagiDataProcessingError)` - If JSON serialization fails (unlikely)
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::JsonStructure;
    ///
    /// let json_struct = JsonStructure::from_json_string(r#"{"key": "value"}"#.to_string()).unwrap();
    /// let json_string = json_struct.copy_as_json_string().unwrap();
    /// assert!(json_string.contains("key"));
    /// ```
    pub fn copy_as_json_string(&self) -> Result<String, FeagiDataProcessingError> {
        Ok(self.json.to_string())
    }
    
    /// Returns a reference to the internal serde_json::Value.
    ///
    /// Provides direct access to the underlying JSON value for reading and
    /// manipulation using the full `serde_json` API. This allows for efficient
    /// access to nested values without serialization overhead.
    ///
    /// # Returns
    /// Reference to the internal `serde_json::Value`
    ///
    /// # Example
    /// ```rust
    /// use serde_json::json;
    /// use feagi_core_data_structures_and_processing::io_data::JsonStructure;
    ///
    /// let json_struct = JsonStructure::from_json_value(json!({"count": 42}));
    /// let json_ref = json_struct.borrow_json_value();
    /// assert_eq!(json_ref["count"], 42);
    /// ```
    pub fn borrow_json_value(&self) -> &serde_json::Value {
        &self.json
    }
}
