//! Multi-structure container serialization for FEAGI data structures.
//!
//! This module provides a container serialization format that can hold multiple different
//! FEAGI structures in a single serialized byte stream. It's designed for batching operations,
//! reducing serialization overhead, and creating compound data structures that contain
//! multiple related pieces of information.
//!
//! ## Format Structure
//!
//! The multi-struct holder uses a sophisticated layout with sub-headers for efficient access:
//!
//! ```text
//! [Global Header: 2 bytes]
//! [Count: 1 byte]
//! [Sub-header 1: 8 bytes][Sub-header 2: 8 bytes]...[Sub-header N: 8 bytes]
//! [Structure 1 Data][Structure 2 Data]...[Structure N Data]
//! ```
//!
//! ### Header Details
//! - **Global Header**: [Type ID: 9][Version: 1]
//! - **Count**: Number of contained structures (max 255)
//! - **Sub-headers**: Each contains [Position: 4 bytes][Length: 4 bytes] in little-endian format
//!
//! ## Usage Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::byte_structures::serializers::{
//!     FeagiByteSerializer, b009_multi_struct_holder::MultiStructSerializerV1
//! };
//!
//! // Create a container with multiple serializers
//! let mut container = MultiStructSerializerV1::new();
//! // container.add_serializer(Box::new(some_json_serializer));
//! // container.add_serializer(Box::new(some_neuron_serializer));
//!
//! // Serialize all contained structures
//! let serialized = container.serialize_new().unwrap();
//! ```
//!
//! ## Benefits
//!
//! - **Reduced I/O**: Single operation to serialize multiple structures
//! - **Atomic Operations**: All structures serialized together or none at all
//! - **Efficient Storage**: Minimal overhead for container metadata
//! - **Random Access**: Sub-headers enable direct access to individual structures
//! - **Type Preservation**: Each contained structure maintains its own format ID

use crate::error::DataProcessingError;
use super::FeagiByteSerializer;
use byteorder::{ByteOrder, LittleEndian};
use crate::byte_structures::feagi_full_byte_data::FeagiFullByteData;

/// Multi-structure container serializer for FEAGI byte structures (Format Type 9, Version 1).
///
/// This serializer acts as a container that can hold multiple different FEAGI serializers
/// and serialize them together into a single byte stream. Each contained serializer
/// maintains its own format and can be individually extracted during deserialization.
///
/// ## Format Details
///
/// - **Format ID**: 9
/// - **Version**: 1
/// - **Max Structures**: 255 (limited by 1-byte count field)
/// - **Byte Order**: Little-endian for all multi-byte values
///
/// ## Memory Layout
///
/// The serializer creates a compound structure where each contained serializer's data
/// is preceded by metadata indicating its position and length within the overall structure.
/// This enables efficient random access during deserialization.
///
/// ## Thread Safety
///
/// This struct is not thread-safe. The contained serializers may not be Send/Sync,
/// and internal state is modified during serialization operations.
pub struct MultiStructSerializerV1 {
    /// Vector of boxed serializers that implement the FeagiByteSerializer trait.
    /// Each serializer will be called during the serialization process to generate
    /// its portion of the overall byte stream.
    contained_serializers: Vec<Box<dyn FeagiByteSerializer>>,
}

impl FeagiByteSerializer for MultiStructSerializerV1 {
    
    /// Returns the format identifier for multi-structure container serialization.
    ///
    /// # Returns
    /// 
    /// Always returns `9` to identify this as a multi-struct holder format.
    fn get_id(&self) -> u8 { 9 }
    
    /// Returns the version number for this multi-struct serializer implementation.
    ///
    /// # Returns
    /// 
    /// Always returns `1` for this version of the multi-struct serializer.
    fn get_version(&self) -> u8 { 1 }
    
    /// Calculates the maximum possible size when all contained structures are serialized.
    ///
    /// This includes:
    /// - Global header (2 bytes)
    /// - Structure count (1 byte) 
    /// - Sub-headers for each structure (8 bytes each)
    /// - Maximum possible size of each contained serializer
    ///
    /// # Returns
    /// 
    /// Total number of bytes required to serialize all contained structures with overhead
    fn get_max_possible_size_when_serialized(&self) -> usize {
        let mut output = 3; // 1 byte for the number contained struct header + global headers
        for serializer in &self.contained_serializers {
            output += serializer.get_max_possible_size_when_serialized() + Self::SUBHEADER_SIZE;
        }
        output
    }

    /// Serializes all contained structures into a newly allocated byte vector.
    ///
    /// This method creates a comprehensive byte stream containing all nested serializers.
    /// The process involves:
    /// 1. Writing the global header and structure count
    /// 2. Reserving space for sub-headers
    /// 3. Serializing each contained structure
    /// 4. Writing sub-headers with position and length information
    ///
    /// # Returns
    /// 
    /// - `Ok(Vec<u8>)`: Successfully serialized compound structure
    /// - `Err(DataProcessingError)`: One or more contained serializers failed
    ///
    /// # Errors
    /// 
    /// Propagates any errors from contained serializers during their individual
    /// serialization processes.
    fn serialize_new(&self) -> Result<FeagiFullByteData, DataProcessingError> {
        let max_size = self.get_max_possible_size_when_serialized();
        let mut output = vec![0u8; max_size];
        
        _ = self.in_place_serialize(&mut output)?;
        Ok(FeagiFullByteData::new(output)?)
    }

    /// Serializes all contained structures into an existing byte buffer.
    ///
    /// This method performs the same serialization as `serialize_new()` but writes
    /// directly into a provided buffer rather than allocating new memory.
    ///
    /// # Arguments
    /// 
    /// * `bytes_to_overwrite` - Mutable byte slice to write serialized data into
    ///
    /// # Returns
    /// 
    /// - `Ok(usize)`: Number of unused bytes remaining in the buffer
    /// - `Err(DataProcessingError)`: Buffer too small or serialization failed
    ///
    /// # Errors
    /// 
    /// - `IncompatibleInplace`: Buffer too small for all contained structures
    /// - Propagated errors from contained serializers
    fn in_place_serialize(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {
        let max_size = self.get_max_possible_size_when_serialized();
        if bytes_to_overwrite.len() < max_size {
            return Err(DataProcessingError::IncompatibleInplace(format!("Multi struct likely requires {} bytes to allocate internal data, but only {} bytes of space was given", max_size, bytes_to_overwrite.len())));
        }

        // Write header
        bytes_to_overwrite[0] = self.get_id();
        bytes_to_overwrite[1] = self.get_version();
        bytes_to_overwrite[2] = self.contained_serializers.len() as u8;

        let mut subheader_write_index: usize = 3;
        let data_start_position: usize = 3 + (self.contained_serializers.len() * Self::SUBHEADER_SIZE);
        let mut data_write_index: usize = data_start_position;

        // Serialize each contained structure and write sub-headers
        for serializer in &self.contained_serializers {
            // Write data to array
            let estimated_data_size = serializer.get_max_possible_size_when_serialized();
            let wasted_byte_count = serializer.in_place_serialize(&mut bytes_to_overwrite[data_write_index..data_write_index + estimated_data_size])?;
            let true_data_size = estimated_data_size - wasted_byte_count;

            // Write sub-header: position (4 bytes) + length (4 bytes)
            LittleEndian::write_u32(&mut bytes_to_overwrite[subheader_write_index..subheader_write_index + 4], data_write_index as u32); // where to start reading
            LittleEndian::write_u32(&mut bytes_to_overwrite[subheader_write_index + 4..subheader_write_index + 8], true_data_size as u32); // how long to read
            
            // Update positions for next iteration
            subheader_write_index += Self::SUBHEADER_SIZE;
            data_write_index += true_data_size;
        }
        let true_number_written_bytes = data_write_index;
        Ok(max_size - true_number_written_bytes)
    }
}

impl MultiStructSerializerV1 {
    const SUBHEADER_SIZE: usize = 8; // each contained struct gets a u32 for describing start index, and a u32 for describing struct length
    
    /// Creates a new empty multi-structure serializer container.
    ///
    /// The container starts empty and serializers can be added using `add_serializer()`.
    /// 
    /// # Returns
    /// 
    /// A new empty MultiStructSerializerV1 instance
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::byte_structures::serializers::b009_multi_struct_holder::MultiStructSerializerV1;
    /// 
    /// let container = MultiStructSerializerV1::new();
    /// assert_eq!(container.len(), 0);
    /// ```
    pub fn new() -> Self {
        MultiStructSerializerV1 {
            contained_serializers: Vec::new(),
        }
    }
    
    /// Adds a serializer to the container.
    ///
    /// The serializer will be included in the compound structure when serialization
    /// is performed. Serializers are processed in the order they are added.
    ///
    /// # Arguments
    /// 
    /// * `serializer` - A boxed serializer implementing FeagiByteSerializer
    ///
    /// # Panics
    /// 
    /// Panics if adding this serializer would exceed the maximum of 255 contained
    /// structures (limited by the 1-byte count field).
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::byte_structures::serializers::{
    ///     b009_multi_struct_holder::MultiStructSerializerV1,
    ///     b001_json::JsonSerializerV1
    /// };
    /// use serde_json::json;
    /// 
    /// let mut container = MultiStructSerializerV1::new();
    /// let json_serializer = JsonSerializerV1::from_json(json!({"test": "data"})).unwrap();
    /// container.add_serializer(Box::new(json_serializer));
    /// ```
    pub fn add_serializer(&mut self, serializer: Box<dyn FeagiByteSerializer>) {
        assert!(self.contained_serializers.len() < 255, "Cannot add more than 255 serializers to MultiStructHolder");
        self.contained_serializers.push(serializer);
    }
    
    /// Returns the number of serializers currently contained in this container.
    ///
    /// # Returns
    /// 
    /// The count of contained serializers (0-255)
    pub fn len(&self) -> usize {
        self.contained_serializers.len()
    }
    
    /// Returns true if the container contains no serializers.
    ///
    /// # Returns
    /// 
    /// `true` if empty, `false` if it contains any serializers
    pub fn is_empty(&self) -> bool {
        self.contained_serializers.is_empty()
    }
    
    
}