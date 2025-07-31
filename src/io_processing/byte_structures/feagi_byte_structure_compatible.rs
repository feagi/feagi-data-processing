//! Trait definition for types that can be serialized to and from FEAGI byte structures.
//!
//! This module defines the `FeagiByteStructureCompatible` trait, which provides a standardized
//! interface for converting Rust types to and from the FEAGI binary serialization format.

use crate::error::{FeagiBytesError, FeagiDataProcessingError};
use super::{FeagiByteStructure, FeagiByteStructureType};

/// Trait for types that can be serialized to and deserialized from FEAGI byte structures.
///
/// This trait enables seamless conversion between Rust types and the FEAGI binary format,
/// providing both serialization (to bytes) and deserialization (from bytes) capabilities.
/// Types implementing this trait can participate in FEAGI's standardized data exchange
/// protocols.
///
/// # Core Capabilities
///
/// - **Type Identity**: Each implementation declares its format type and version
/// - **Serialization**: Convert Rust objects to FEAGI byte format
/// - **Deserialization**: Create Rust objects from FEAGI byte format
/// - **Size Estimation**: Calculate required buffer sizes for efficient allocation
/// - **Validation**: Ensure sufficient buffer space before serialization
///
/// # Serialization Format
///
/// All compatible types must follow the FEAGI byte structure format:
/// ```text
/// [Type ID (1 byte)][Version (1 byte)][Type-specific payload...]
/// ```
///
/// # Implementation Requirements
///
/// Implementors must provide:
/// - Format type and version identification
/// - Serialization logic that writes to a provided byte slice
/// - Deserialization logic that reconstructs objects from byte structures
/// - Accurate size estimation for buffer allocation
/// 
/// # Thread Safety
///
/// Implementations should be thread-safe, as byte structures may be
/// processed concurrently in multi-threaded FEAGI environments.
pub trait FeagiByteStructureCompatible {

    /// Returns the FEAGI byte structure type identifier for this implementation.
    ///
    /// This identifier is used as the first byte in the serialized format to
    /// indicate what type of data structure is contained in the byte stream.
    fn get_type(&self) -> FeagiByteStructureType;

    /// Returns the version number for this implementation's serialization format.
    ///
    /// Version numbers allow for format evolution while maintaining backward
    /// compatibility. This value is stored as the second byte in the serialized format.
    fn get_version(&self) -> u8;

    /// Serializes this object into the provided byte slice.
    ///
    /// Writes the complete FEAGI byte structure representation (including header)
    /// into the provided mutable slice. The implementation should write the type
    /// identifier, version, and all object data.
    ///
    /// # Arguments
    /// * `slice` - Mutable byte slice to write into (must have sufficient capacity)
    ///
    /// # Returns
    /// * `Ok(usize)` - Number of bytes actually written to the slice
    /// * `Err(FeagiDataProcessingError)` - If serialization fails or slice is too small
    ///
    /// # Requirements
    /// - Must call `verify_slice_has_enough_space()` before writing
    /// - Must write exactly the format specified by `get_type()` and `get_version()`
    /// - Should write deterministic output for the same object state
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, FeagiDataProcessingError>;

    /// Returns the maximum number of bytes needed to serialize this object.
    ///
    /// This should return the worst-case buffer size needed for serialization,
    /// including the 2-byte header (type + version) plus all object data.
    /// Used for efficient buffer allocation.
    ///
    /// # Returns
    /// Maximum byte count needed for complete serialization (including header)
    fn max_number_bytes_needed(&self) -> usize;

    /// Creates a new instance of this type from a FEAGI byte structure.
    ///
    /// Deserializes a complete object from the provided byte structure,
    /// validating the format and reconstructing the original object state.
    ///
    /// # Arguments
    /// * `feagi_byte_structure` - Source byte structure containing serialized data
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully deserialized object
    /// * `Err(FeagiDataProcessingError)` - If deserialization fails due to:
    ///   - Invalid format or corrupted data
    ///   - Type/version mismatch
    ///   - Insufficient data in the byte structure
    ///
    /// # Implementation Notes
    /// - Should handle any format-specific validation requirements
    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, FeagiDataProcessingError> where Self: Sized;

    /// Validates that a byte slice has sufficient space for serialization.
    ///
    /// This helper method checks if the provided slice can accommodate the
    /// complete serialized representation of this object. Should be called
    /// before attempting serialization.
    ///
    /// # Arguments
    /// * `slice` - Byte slice to validate for sufficient capacity
    ///
    /// # Returns
    /// * `Ok(())` - Slice has sufficient space
    /// * `Err(FeagiBytesError)` - Slice is too small, with details about required vs available space
    fn verify_slice_has_enough_space(&self, slice: &[u8]) -> Result<(), FeagiBytesError> {
        if slice.len() < self.max_number_bytes_needed() {
            return Err(FeagiBytesError::UnableToValidateBytes(format!("Given slice is only {} bytes long when {} bytes of space are required!", slice.len(), self.max_number_bytes_needed())));
        }
        Ok(())
    }

    /// Creates a new FeagiByteStructure containing this object's serialized data.
    ///
    /// This convenience method handles the complete serialization process:
    /// 1. Allocates a buffer of the appropriate size
    /// 2. Serializes this object into the buffer
    /// 3. Creates and validates a FeagiByteStructure from the result
    ///
    /// # Returns
    /// * `Ok(FeagiByteStructure)` - Complete byte structure ready for transmission/storage
    /// * `Err(FeagiDataProcessingError)` - If serialization or validation fails
    ///
    /// # Performance Notes
    /// This method allocates a new buffer sized to `max_number_bytes_needed()`.
    /// Some bytes may be unused if the actual serialized size is smaller than
    /// the maximum estimate.
    fn as_new_feagi_byte_structure(&self) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        let mut bytes: Vec<u8> = vec![0; self.max_number_bytes_needed()];
        _ = self.overwrite_feagi_byte_structure_slice(&mut bytes)?; // theoretically some bytes may be wasted
        FeagiByteStructure::create_from_bytes(bytes)
    }

}