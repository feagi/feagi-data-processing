//! Multi-structure container deserialization for FEAGI data structures.
//!
//! This module provides deserialization capabilities for multi-structure container formats
//! that hold multiple different FEAGI structures in a single byte stream.
//!
//! ## Format Support
//!
//! - **Format Type**: 9 (Multi-Struct Holder)
//! - **Supported Versions**: 1
//! - **Max Contained Structures**: 255
//! - **Byte Order**: Little-endian
//!
//! ## Container Structure
//!
//! The deserializer handles byte streams with this layout:
//! ```text
//! [Global Header: 2 bytes]
//! [Structure Count: 1 byte]
//! [Sub-header 1: 8 bytes][Sub-header 2: 8 bytes]...[Sub-header N: 8 bytes]
//! [Structure 1 Data][Structure 2 Data]...[Structure N Data]
//! ```
//!
//! Each sub-header contains position and length information enabling direct access
//! to individual contained structures without parsing the entire container.
//!
//! ## Usage Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::byte_structures::deserializers::{
//!     b009_multi_struct_holder::MultiStructHolderDeserializerV1,
//!     Deserializer
//! };
//!
//! // Deserialize container holding multiple structures
//! let container_bytes: &[u8] = /* serialized container data */;
//! let deserializer = MultiStructHolderDeserializerV1::from_data_slice(container_bytes).unwrap();
//! let contained_structures = deserializer.to_multiple_structs().unwrap();
//!
//! // Process each contained structure individually
//! for structure in contained_structures {
//!     match structure {
//!         Deserializer::JsonV1(json_des) => { /* handle JSON */ },
//!         Deserializer::NeuronCategoricalXYZPV1(neuron_des) => { /* handle neurons */ },
//!         // ... handle other types
//!     }
//! }
//! ```

use byteorder::{ByteOrder, LittleEndian};
use crate::error::DataProcessingError;
use super::{FeagiByteDeserializer, FeagiByteStructureType, verify_header_of_full_structure_bytes, Deserializer, build_deserializer};

/// Multi-structure container deserializer for FEAGI byte structures (Format Type 9, Version 1).
///
/// This deserializer processes byte streams containing multiple different FEAGI structures
/// serialized together by the corresponding [`MultiStructSerializerV1`]. It provides
/// efficient access to individual contained structures through sub-header indexing.
///
/// ## Lifetime Management
///
/// The deserializer maintains a reference to the original byte data, enabling zero-copy
/// access to contained structures. Individual structures are extracted on-demand when
/// `to_multiple_structs()` is called.
///
/// ## Format Validation
///
/// The deserializer validates:
/// - Global header format and version
/// - Container structure count limits
/// - Sub-header consistency and bounds
/// - Individual structure header validity

pub struct MultiStructHolderDeserializerV1<'internal_bytes> {
    /// Reference to the complete byte slice containing the multi-structure container.
    /// This includes the global header, structure count, sub-headers, and all contained structure data.
    data_slice: &'internal_bytes [u8],
}

impl FeagiByteDeserializer for MultiStructHolderDeserializerV1<'_> {
    /// Returns the format identifier for multi-structure container deserialization.
    ///
    /// # Returns
    /// 
    /// Always returns the value corresponding to `FeagiByteStructureType::MultiStructHolder` (9)
    fn get_id(&self) -> u8 {FeagiByteStructureType::MultiStructHolder as u8}
    
    /// Returns the version number for this multi-struct deserializer implementation.
    ///
    /// # Returns
    /// 
    /// Always returns `1` for this version of the multi-struct deserializer
    fn get_version(&self) -> u8 {1}
}

impl<'internal_bytes> MultiStructHolderDeserializerV1<'internal_bytes> {
    /// Creates a new multi-structure container deserializer from a byte slice.
    ///
    /// This constructor validates the global header to ensure the data is compatible
    /// with multi-structure container deserialization. It verifies both the format
    /// type and version before creating the deserializer instance.
    ///
    /// # Arguments
    /// 
    /// * `data_slice` - Byte slice containing the complete container structure with global header
    ///
    /// # Returns
    /// 
    /// - `Ok(MultiStructHolderDeserializerV1)`: Successfully created deserializer
    /// - `Err(DataProcessingError)`: Header validation failed
    ///
    /// # Errors
    /// 
    /// - `InvalidByteStructure`: Data too short, wrong format type, or wrong version
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::byte_structures::deserializers::b009_multi_struct_holder::MultiStructHolderDeserializerV1;
    /// 
    /// // Valid container byte structure (minimal example)
    /// let container_data = [9u8, 1u8, 0u8]; // Type 9, Version 1, 0 contained structures
    /// let deserializer = MultiStructHolderDeserializerV1::from_data_slice(&container_data).unwrap();
    /// 
    /// // Invalid format type will fail
    /// let invalid_data = [8u8, 1u8, 0u8];
    /// assert!(MultiStructHolderDeserializerV1::from_data_slice(&invalid_data).is_err());
    /// ```
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<MultiStructHolderDeserializerV1<'internal_bytes>, DataProcessingError> {
        verify_header_of_full_structure_bytes(data_slice, FeagiByteStructureType::MultiStructHolder, 1)?;
        Ok(MultiStructHolderDeserializerV1 { data_slice })
    }
    
    /// Extracts and deserializes all contained structures from the container.
    ///
    /// This method processes the container's sub-headers to locate each contained structure,
    /// then creates appropriate deserializers for each one. The result is a vector of
    /// individual deserializers that can be processed independently.
    ///
    /// # Returns
    /// 
    /// - `Ok(Vec<Deserializer>)`: Successfully extracted all contained structures
    /// - `Err(DataProcessingError)`: Container parsing failed or contained structure invalid
    ///
    /// # Errors
    /// 
    /// - `InvalidByteStructure`: Container too short, invalid sub-headers, or malformed contained structures
    /// - Propagated errors from individual structure deserialization
    ///
    /// # Process
    /// 
    /// 1. Reads the structure count from the container header
    /// 2. Validates that the container is large enough for all sub-headers
    /// 3. Iterates through sub-headers to extract position/length information
    /// 4. Creates individual deserializers for each contained structure
    /// 5. Validates bounds and data integrity for each structure
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::byte_structures::deserializers::{
    ///     b009_multi_struct_holder::MultiStructHolderDeserializerV1,
    ///     Deserializer
    /// };
    /// 
    /// // Assuming container_bytes contains a valid multi-structure container
    /// let container_bytes: &[u8] = /* container data */;
    /// let deserializer = MultiStructHolderDeserializerV1::from_data_slice(container_bytes).unwrap();
    /// 
    /// // Extract all contained structures
    /// let structures = deserializer.to_multiple_structs().unwrap();
    /// println!("Container holds {} structures", structures.len());
    /// 
    /// // Process each structure by type
    /// for (index, structure) in structures.iter().enumerate() {
    ///     match structure {
    ///         Deserializer::JsonV1(_) => println!("Structure {} is JSON", index),
    ///         Deserializer::NeuronCategoricalXYZPV1(_) => println!("Structure {} is neuron data", index),
    ///         Deserializer::MultiStructHolderV1(_) => println!("Structure {} is nested container", index),
    ///     }
    /// }
    /// ```
    pub fn to_multiple_structs(&self) -> Result<Vec<Deserializer> , DataProcessingError> {
        const SUB_HEADER_SIZE_PER_STRUCT: usize = 8;
        let number_contained_structs: usize = self.data_slice[2] as usize; // This header element is the count as a u8
        let minimum_number_of_bytes_for_headers: usize = crate::byte_structures::GLOBAL_HEADER_SIZE + 1 + (number_contained_structs * SUB_HEADER_SIZE_PER_STRUCT);
        
        if self.data_slice.len() < minimum_number_of_bytes_for_headers {
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte structure for MultiStructHolderV1 needs a length of {} to fit just the cortical details header, but is a length of {}",
                                                                         minimum_number_of_bytes_for_headers, self.data_slice.len())));
        }
        
        let mut output: Vec<Deserializer> = Vec::with_capacity(number_contained_structs);
        let mut reading_index: usize = 3;
        
        for i in 0..number_contained_structs {
            let data_start_reading: usize = LittleEndian::read_u32(&self.data_slice[reading_index..reading_index+4]) as usize;
            let number_bytes_to_read: usize = LittleEndian::read_u32(&self.data_slice[reading_index+4..reading_index+8]) as usize;

            if self.data_slice.len() < minimum_number_of_bytes_for_headers + data_start_reading + number_bytes_to_read {
                return Err(DataProcessingError::InvalidByteStructure("Byte structure for MultiStructHolderV1 is too short to fit the data the header says it contains!".into()));
            }
            output.push(build_deserializer(&self.data_slice[data_start_reading..data_start_reading+number_bytes_to_read])?);
        }
        
        Ok(output)
    }
}
