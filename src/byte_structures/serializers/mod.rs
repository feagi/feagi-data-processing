//! Serialization implementations for FEAGI data structures.
//!
//! This module contains serializer implementations for converting FEAGI data structures
//! into various byte formats. All serializers implement the [`FeagiByteSerializer`] trait
//! to provide a consistent interface across different serialization formats.
//!
//! ## Available Serializers
//!
//! - [`b001_json`]: JSON serialization for human-readable output
//! - [`b009_multi_struct_holder`]: Container format for multiple structures
//! - [`b011_neuron_categorical_xyzp`]: Efficient binary format for neuron data
//!
//! ## Trait Design
//!
//! The [`FeagiByteSerializer`] trait provides both new allocation and in-place
//! serialization methods:
//!
//! - `serialize_new()`: Creates a new Vec<u8> with the serialized data
//! - `serialize_in_place()`: Writes serialized data to an existing buffer
//!
//! ## Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::byte_structures::serializers::FeagiByteSerializer;
//!
//! // Using a serializer (example structure)
//! # struct ExampleSerializer;
//! # impl FeagiByteSerializer for ExampleSerializer {
//! #     fn get_id(&self) -> u8 { 1 }
//! #     fn get_version(&self) -> u8 { 1 }
//! #     fn get_max_possible_size_when_serialized(&self) -> usize { 100 }
//! #     fn serialize_new(&self) -> Result<Vec<u8>, feagi_core_data_structures_and_processing::error::DataProcessingError> { Ok(vec![]) }
//! #     fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, feagi_core_data_structures_and_processing::error::DataProcessingError> { Ok(0) }
//! # }
//! let serializer = ExampleSerializer;
//! 
//! // Serialize to new vector
//! let serialized = serializer.serialize_new().unwrap();
//! 
//! // Or serialize in-place to existing buffer
//! let mut buffer = vec![0u8; serializer.get_max_possible_size_when_serialized()];
//! let bytes_written = serializer.serialize_in_place(&mut buffer).unwrap();
//! ```

/// JSON serialization implementation (format type 1).
/// 
/// Provides human-readable JSON serialization of FEAGI data structures.
/// Suitable for debugging, configuration files, and scenarios where
/// readability is prioritized over performance.
pub mod b001_json;

/// Neuron categorical XYZP binary serialization implementation (format type 11).
/// 
/// Highly optimized binary serialization format specifically designed for
/// neuron data with X, Y, Z coordinates and potential values. Provides
/// maximum efficiency for large neuron datasets.
pub mod b011_neuron_categorical_xyzp;

/// Multi-structure container serialization implementation (format type 9).
/// 
/// Container format that can hold multiple different FEAGI structures
/// in a single serialized byte stream. Useful for batching operations
/// and reducing serialization overhead.
pub mod b009_multi_struct_holder;

use crate::error::DataProcessingError;

/// Trait defining the interface for all FEAGI byte structure serializers.
/// 
/// This trait provides a consistent interface for serializing FEAGI data structures
/// into byte formats. It supports both allocation-based and in-place serialization
/// methods to accommodate different performance requirements.
/// 
/// ## Implementation Requirements
/// 
/// Implementors must provide:
/// - Unique format ID and version numbers
/// - Size estimation for buffer allocation
/// - Both new allocation and in-place serialization methods
/// 
/// ## Thread Safety
/// 
/// While not currently enforced by trait bounds, implementations should consider
/// thread safety requirements for concurrent usage scenarios.
pub trait FeagiByteSerializer{ // : Send + Sync 
    /// Returns the unique format identifier for this serializer.
    /// 
    /// This byte value is used in the global header to identify the serialization
    /// format when deserializing data. Must be unique across all serializer types.
    fn get_id(&self) -> u8;
    
    /// Returns the version number for this serializer implementation.
    /// 
    /// Version numbers allow for format evolution while maintaining backward
    /// compatibility. Should be incremented when breaking changes are made
    /// to the serialization format.
    fn get_version(&self) -> u8;
    
    /// Returns the maximum possible size in bytes when this structure is serialized.
    /// 
    /// This is used for buffer allocation in in-place serialization scenarios.
    /// Should return a conservative upper bound to avoid buffer overflow errors.
    /// 
    /// # Returns
    /// 
    /// Maximum number of bytes that could be produced by serialization
    fn get_max_possible_size_when_serialized(&self) -> usize;
    
    /// Serializes the data structure into a newly allocated byte vector.
    /// 
    /// This method handles memory allocation internally and returns a vector
    /// containing the complete serialized data including the global header.
    /// 
    /// # Returns
    /// 
    /// - `Ok(Vec<u8>)`: Successfully serialized data
    /// - `Err(DataProcessingError)`: Serialization failed
    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError>;
    
    /// Serializes the data structure into an existing byte buffer.
    /// 
    /// This method writes serialized data directly into the provided buffer,
    /// which must be large enough to hold the complete serialized structure.
    /// The buffer size can be determined using `get_max_possible_size_when_serialized()`.
    /// 
    /// # Arguments
    /// 
    /// * `bytes_to_overwrite` - Mutable byte slice to write serialized data into
    /// 
    /// # Returns
    /// 
    /// - `Ok(usize)`: Number of bytes actually written to the buffer
    /// - `Err(DataProcessingError)`: Serialization failed or buffer too small
    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError>;
    
    /// Generates the standard 2-byte global header for this serializer.
    /// 
    /// The global header contains the format ID and version number that
    /// prefix all serialized FEAGI data structures. This method provides
    /// a default implementation that should be suitable for most serializers.
    /// 
    /// # Returns
    /// 
    /// 2-byte array containing [format_id, version]
    fn generate_global_header(&self) ->[u8; 2] {
        [self.get_id(), self.get_version()]
    }
}