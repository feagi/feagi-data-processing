//! Neuron categorical XYZP binary serialization for FEAGI neural data.
//!
//! This module provides highly optimized binary serialization specifically designed for
//! neuron data with X, Y, Z coordinates and potential (P) values. It organizes neuron
//! data by cortical areas and provides the most efficient serialization format for
//! large-scale neural datasets in the FEAGI system.
//!
//! ## Format Structure
//!
//! The serializer creates a compact binary format optimized for neural data:
//!
//! ```text
//! [Global Header: 2 bytes]
//! [Cortical Count: 2 bytes]
//! [Cortical Descriptor 1: 14 bytes][Cortical Descriptor 2: 14 bytes]...[Cortical Descriptor N: 14 bytes]  
//! [Neuron Data 1][Neuron Data 2]...[Neuron Data N]
//! ```
//!
//! ### Header Details
//! - **Global Header**: [Type ID: 11][Version: 1]
//! - **Cortical Count**: Number of cortical areas (little-endian u16)
//! - **Cortical Descriptors**: [Cortical ID: 6 bytes][Data Position: 4 bytes][Data Length: 4 bytes]
//! - **Neuron Data**: Packed XYZP arrays in quarters (all X values, then all Y, then all Z, then all P)
//!
//! ## Memory Layout
//!
//! Neuron data is stored in a "struct of arrays" format for optimal memory access patterns:
//! - First quarter: All X coordinates (u32 each)
//! - Second quarter: All Y coordinates (u32 each)  
//! - Third quarter: All Z coordinates (u32 each)
//! - Fourth quarter: All potential values (f32 each)
//!
//! ## Performance Benefits
//!
//! - **Cache Efficiency**: Sequential access to same data types
//! - **SIMD Friendly**: Aligned data enables vectorized operations
//! - **Compact Storage**: No padding or alignment waste
//! - **Fast Deserialization**: Direct memory mapping possible

use byteorder::{ByteOrder, LittleEndian};
use crate::byte_structures::GLOBAL_HEADER_SIZE;
use crate::error::DataProcessingError;
use crate::neuron_data::{CorticalMappedNeuronData, NeuronXYZPArrays};
use crate::byte_structures::feagi_full_byte_data::FeagiFullByteData;
use super::FeagiByteSerializer;

/// Neuron categorical XYZP binary serializer for FEAGI neural data (Format Type 11, Version 1).
///
/// This serializer provides the most efficient binary representation for neuron data
/// in the FEAGI system. It organizes neurons by cortical areas and uses a compact
/// "struct of arrays" memory layout for optimal performance during serialization,
/// deserialization, and subsequent processing.
///
/// ## Format Details
///
/// - **Format ID**: 11
/// - **Version**: 1
/// - **Byte Order**: Little-endian for all multi-byte values
/// - **Coordinate System**: Cartesian (X, Y, Z) coordinates
/// - **Data Types**: u32 for coordinates, f32 for potentials
///
/// ## Memory Efficiency
///
/// The serializer uses a quarter-based layout where all values of the same type
/// are stored together, enabling:
/// - Better CPU cache utilization
/// - SIMD optimization opportunities  
/// - Reduced memory fragmentation
/// - Faster bulk operations
///
/// ## Cortical Organization
///
/// Neural data is organized by cortical areas, each identified by a 6-character
/// ASCII cortical ID. This enables efficient routing and processing of neural
/// signals within the FEAGI brain simulation.
pub struct NeuronCategoricalXYZPSerializerV1 {
    /// Neuron data organized by cortical areas. Each cortical area is identified
    /// by a unique CorticalID and contains arrays of neuron coordinates and potentials.
    cortical_mapped_neuron_data: CorticalMappedNeuronData,
}

impl FeagiByteSerializer for NeuronCategoricalXYZPSerializerV1 {
    /// Returns the format identifier for neuron categorical XYZP serialization.
    ///
    /// # Returns
    /// 
    /// Always returns `11` to identify this as a neuron XYZP binary format.
    fn get_id(&self) -> u8 { 11 }
    
    /// Returns the version number for this neuron serializer implementation.
    ///
    /// # Returns
    /// 
    /// Always returns `1` for this version of the neuron XYZP serializer.
    fn get_version(&self) -> u8 { 1 }
    
    /// Calculates the maximum possible size when all neuron data is serialized.
    ///
    /// This includes:
    /// - Global header (2 bytes)
    /// - Cortical count header (2 bytes)
    /// - Cortical descriptors (14 bytes each)
    /// - Neuron data (16 bytes per neuron: 4×u32 + 4×f32)
    ///
    /// # Returns
    /// 
    /// Total number of bytes required for all cortical areas and their neuron data
    fn get_max_possible_size_when_serialized(&self) -> usize {

        let mut size = GLOBAL_HEADER_SIZE + Self::CORTICAL_COUNT_HEADER_SIZE;
        for (_cortical_id, mapped_neuron_data) in self.cortical_mapped_neuron_data.iter(){
            size += NeuronXYZPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE + mapped_neuron_data.get_number_of_bytes_needed_if_serialized()
        };
        size // This is precise, no overestimation
    }
    
    /// Serializes all neuron data into a newly allocated byte vector.
    ///
    /// Creates a compact binary representation of all cortical areas and their
    /// associated neuron data. The serialization process:
    /// 1. Writes global header with format ID and version
    /// 2. Writes cortical area count
    /// 3. Writes cortical descriptors with data positions and lengths
    /// 4. Writes neuron data in quarter-based layout for each cortical area
    ///
    /// # Returns
    /// 
    /// - `Ok(Vec<u8>)`: Successfully serialized neural data
    /// - `Err(DataProcessingError)`: Serialization failed (e.g., cortical ID write error)
    ///
    /// # Errors
    /// 
    /// - `InvalidByteStructure`: Issues with cortical ID formatting
    /// - Propagated errors from internal neuron data serialization
    fn serialize_new(&self) -> Result<FeagiFullByteData, DataProcessingError> {
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

        let mut vector_data: Vec<u8> = Vec::with_capacity(self.get_max_possible_size_when_serialized());
        _ = self.in_place_serialize(&mut vector_data);

        Ok(FeagiFullByteData::new(vector_data)?)

    }

    fn in_place_serialize(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {
        let num_bytes_needed: usize = self.get_max_possible_size_when_serialized();
        if bytes_to_overwrite.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store neuron XYZP data! Need {} bytes but given {}!", num_bytes_needed, bytes_to_overwrite.len())));
        }

        bytes_to_overwrite[0] = self.get_id();
        bytes_to_overwrite[1] = self.get_version();

        let number_cortical_areas: usize = self.cortical_mapped_neuron_data.len();
        LittleEndian::write_u16(&mut bytes_to_overwrite[2..4], number_cortical_areas as u16);

        let mut subheader_write_index: usize = GLOBAL_HEADER_SIZE + Self::CORTICAL_COUNT_HEADER_SIZE;
        let mut neuron_data_write_index: u32 = subheader_write_index as u32 + (number_cortical_areas as u32 * NeuronXYZPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        for (cortical_id, neuron_data) in &self.cortical_mapped_neuron_data {
            // Write cortical subheader
            cortical_id.write_bytes_at(&mut bytes_to_overwrite[subheader_write_index .. subheader_write_index + 6])?;
            let reading_start: u32 = neuron_data_write_index;
            let reading_length: u32 = neuron_data.get_number_of_neurons_used() as u32 * NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON as u32;
            LittleEndian::write_u32(&mut bytes_to_overwrite[subheader_write_index + 6 .. subheader_write_index + 10], reading_start);
            LittleEndian::write_u32(&mut bytes_to_overwrite[subheader_write_index + 10 .. subheader_write_index + 14], reading_length);
            
            // write neuron data
            write_neural_data_to_bytes(neuron_data, &mut bytes_to_overwrite[reading_start as usize .. (reading_start + reading_length) as usize])?;

            // update indexes
            neuron_data_write_index += reading_length;
            subheader_write_index += NeuronXYZPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        };
        let wasted_space = bytes_to_overwrite.len() - num_bytes_needed;
        Ok(wasted_space)
    }

}

impl NeuronCategoricalXYZPSerializerV1 {
    
    /// Size in bytes of the cortical count header field.
    pub const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

    /// Creates a new neuron serializer from cortical mapped neuron data.
    ///
    /// Takes ownership of the provided cortical neuron data and prepares it for
    /// serialization. The data is organized by cortical areas, each containing
    /// arrays of neuron coordinates and potential values.
    ///
    /// # Arguments
    /// 
    /// * `cortical_mapped_neuron_data` - HashMap mapping cortical IDs to neuron data arrays
    ///
    /// # Returns
    /// 
    /// - `Ok(NeuronCategoricalXYZPSerializerV1)`: Successfully created serializer
    /// - `Err(DataProcessingError)`: Currently always succeeds, but returns Result for API consistency
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::{
    ///     byte_structures::serializers::b011_neuron_categorical_xyzp::NeuronCategoricalXYZPSerializerV1,
    ///     neuron_data::CorticalMappedNeuronData
    /// };
    /// 
    /// let mut neuron_data = CorticalMappedNeuronData::new();
    /// // ... populate neuron_data with cortical areas and neurons
    /// 
    /// let serializer = NeuronCategoricalXYZPSerializerV1::from_cortical_mapped_neuron_data(neuron_data).unwrap();
    /// ```
    pub fn from_cortical_mapped_neuron_data(cortical_mapped_neuron_data: CorticalMappedNeuronData) -> Result<NeuronCategoricalXYZPSerializerV1, DataProcessingError> {
        Ok(NeuronCategoricalXYZPSerializerV1{cortical_mapped_neuron_data})
    }

    /// Provides mutable access to the internal cortical mapped neuron data.
    ///
    /// This method allows modification of the neuron data after the serializer has been created.
    /// Useful for updating neuron positions, potentials, or adding/removing cortical areas
    /// before serialization.
    ///
    /// # Returns
    /// 
    /// Mutable reference to the internal CorticalMappedNeuronData
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use feagi_core_data_structures_and_processing::{
    ///     byte_structures::serializers::b011_neuron_categorical_xyzp::NeuronCategoricalXYZPSerializerV1,
    ///     neuron_data::{CorticalMappedNeuronData, NeuronXYZPArrays},
    ///     cortical_data::CorticalID
    /// };
    ///
    /// let neuron_data = CorticalMappedNeuronData::new();
    /// let mut serializer = NeuronCategoricalXYZPSerializerV1::from_cortical_mapped_neuron_data(neuron_data).unwrap();
    ///
    /// // Add or modify neuron data
    /// let data_ref = serializer.as_mut();
    /// let cortical_id = CorticalID::from_str("iv00CC").unwrap();
    /// let neurons = NeuronXYZPArrays::new(1000).unwrap();
    /// data_ref.insert(cortical_id, neurons);
    /// ```
    pub fn as_mut(&mut self) -> &mut CorticalMappedNeuronData {
        &mut self.cortical_mapped_neuron_data
    }
}

/// Writes neuron data to a byte buffer using the quarter-based memory layout.
///
/// This function implements the core serialization logic for individual neuron data arrays.
/// It organizes the data in quarters where all values of the same type are stored together:
/// - First quarter: All X coordinates
/// - Second quarter: All Y coordinates  
/// - Third quarter: All Z coordinates
/// - Fourth quarter: All potential values
///
/// This layout provides excellent cache performance and enables SIMD optimizations
/// during both serialization and subsequent processing.
///
/// # Arguments
/// 
/// * `neurons` - Reference to the neuron data arrays to be serialized
/// * `bytes_to_write_to` - Mutable byte slice to write the serialized data
///
/// # Returns
/// 
/// - `Ok(())`: Successfully wrote all neuron data
/// - `Err(DataProcessingError)`: Buffer size mismatch or other serialization error
///
/// # Errors
/// 
/// Returns `InvalidByteStructure` if the provided buffer size doesn't exactly match
/// the required size for the neuron data (number_of_neurons × 16 bytes).
///
/// # Memory Layout
/// 
/// For N neurons, the byte layout is:
/// ```text
/// [X₁][X₂]...[Xₙ][Y₁][Y₂]...[Yₙ][Z₁][Z₂]...[Zₙ][P₁][P₂]...[Pₙ]
/// ```
/// 
/// Where each coordinate is a 4-byte little-endian u32 and each potential is a 4-byte little-endian f32.
fn write_neural_data_to_bytes(neurons: &NeuronXYZPArrays, bytes_to_write_to: &mut [u8]) -> Result<(), DataProcessingError> {
    const U32_F32_LENGTH: usize = 4;
    let number_of_neurons_to_write: usize = neurons.get_number_of_neurons_used();
    let number_bytes_needed = NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * number_of_neurons_to_write;
    if bytes_to_write_to.len() != number_bytes_needed {
        return Err(DataProcessingError::InvalidByteStructure(format!("Need exactly {} bytes to write xycp neuron data, but given a space of {} bytes!", bytes_to_write_to.len(), number_bytes_needed).into()))
    }
    let mut x_offset: usize = 0;
    let mut y_offset = number_of_neurons_to_write * NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON / 4; // we want to be a quarter way
    let mut c_offset = y_offset * 2; // half way
    let mut p_offset = y_offset * 3; // three quarters way

    let (x, y, c, p) = neurons.borrow_xyzp_vectors();

    for i in 0 .. number_of_neurons_to_write {
        let x_bytes = x[i].to_le_bytes(); // TODO why can this not see the byte length?
        let y_bytes = y[i].to_le_bytes();
        let c_bytes = c[i].to_le_bytes();
        let p_bytes = p[i].to_le_bytes();

        bytes_to_write_to[x_offset .. x_offset + U32_F32_LENGTH].copy_from_slice(&x_bytes);
        bytes_to_write_to[y_offset .. y_offset + U32_F32_LENGTH].copy_from_slice(&y_bytes);
        bytes_to_write_to[c_offset .. c_offset + U32_F32_LENGTH].copy_from_slice(&c_bytes);
        bytes_to_write_to[p_offset .. p_offset + U32_F32_LENGTH].copy_from_slice(&p_bytes);

        x_offset += U32_F32_LENGTH;
        y_offset += U32_F32_LENGTH;
        c_offset += U32_F32_LENGTH;
        p_offset += U32_F32_LENGTH;
    };
    
    Ok(())
}