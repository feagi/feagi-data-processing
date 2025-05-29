//! Provides data structures and functions for handling neuron data in FEAGI.
//! This module contains utilities for managing, transforming, and serializing neuron position
//! and properties data organized by cortical areas.

use std::collections::HashMap;
use bytemuck::{cast_slice, Pod, Zeroable};
use crate::cortical_data::CorticalID;
use crate::error::DataProcessingError;

/// A mapping from cortical IDs to their corresponding neuron data arrays.
/// This structure is used to organize neuron data by cortical area.
pub type CorticalMappedNeuronData = HashMap<CorticalID, NeuronXYCPArrays>;

/// Represents neuron data as four parallel arrays for X, Y, channel, and potential values.
/// This structure provides an efficient memory layout for serialization and processing of neuron data.
#[derive(Clone)]
pub struct NeuronXYCPArrays{
    /// X coordinates of neurons (using Cartesian coordinate system)
    x: Vec<u32>, // Remember, FEAGI is cartesian!
    /// Y coordinates of neurons
    y: Vec<u32>,
    /// Channel indices of neurons
    c: Vec<u32>,
    /// Potential/activation values of neurons
    p: Vec<f32>,
}

impl NeuronXYCPArrays{
    /// Number of bytes used to represent a single neuron in memory (going across x y z p elements)
    pub const NUMBER_BYTES_PER_NEURON: usize = 16;
    pub const PER_CORTICAL_HEADER_DESCRIPTOR_SIZE: usize = 14;
    
    /// Creates a new NeuronXYCPArrays instance with capacity for the specified maximum number of neurons.
    ///
    /// # Arguments
    /// * `maximum_number_of_neurons_possibly_needed` - The maximum number of neurons this structure should be able to hold
    ///
    /// # Returns
    /// * `Result<Self, DataProcessingError>` - A new instance or an error if the input is invalid
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> Result<Self, DataProcessingError> {
        if maximum_number_of_neurons_possibly_needed == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Given number of neurons possible must be greater than 0!".into()));
        };
        Ok(NeuronXYCPArrays{
            x: Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            y: Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            c: Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            p: Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
        })
    }

    /// Creates a new NeuronXYCPArrays from a 3D resolution tuple.
    ///
    /// # Arguments
    /// * `resolution` - A tuple representing the 3D dimensions (neuron count) in the x y z directions 
    ///
    /// # Returns
    /// * `Result<Self, DataProcessingError>` - A new instance with capacity for all neurons in the 3D space
    pub fn new_from_resolution(resolution: (usize, usize, usize)) -> Result<Self, DataProcessingError> {
        NeuronXYCPArrays::new(resolution.0 * resolution.1 * resolution.2)
    }
    
    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self, DataProcessingError> {
        let bytes_length = bytes.len();
        if bytes_length % NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON != 0 {
            return Err(DataProcessingError::InvalidByteStructure("As NeuronXYCPArrays contains 4 internal arrays of equal length, each of elements of 4 bytes each (uint32 and float), the input byte array must be divisible by 16!".into()));
        }
        let x_end = bytes_length / 4;
        let y_end = bytes_length / 2;
        let c_end = x_end * 3;
        
        Ok(NeuronXYCPArrays{
            x: cast_slice::<u8, u32>(&bytes[0..x_end]).to_vec(),
            y: cast_slice::<u8, u32>(&bytes[x_end..y_end]).to_vec(),
            c: cast_slice::<u8, u32>(&bytes[y_end..c_end]).to_vec(),
            p: cast_slice::<u8, f32>(&bytes[c_end..]).to_vec(),
        })
    }
    
    /// Serializes a cortical-mapped neuron data structure into a new byte vector.
    ///
    /// # Arguments
    /// * `mapped_data` - The cortical-mapped neuron data to serialize
    ///
    /// # Returns
    /// * `Result<Vec<u8>, DataProcessingError>` - The serialized byte vector or an error
    pub fn cortical_mapped_neuron_data_to_bytes(mapped_data: &CorticalMappedNeuronData) -> Result<Vec<u8>, DataProcessingError> {
        const BYTE_STRUCT_ID: u8 = 11;
        const BYTE_STRUCT_VERSION: u8 = 1;
        const GLOBAL_HEADER_SIZE: usize = crate::byte_data_functions::GLOBAL_HEADER_SIZE;
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;
        


        // Calculate prerequisite info
        let number_cortical_areas: usize = mapped_data.len();
        let mut number_of_neurons_total: usize = 0;
        for (_, neuron_data) in mapped_data {
            number_of_neurons_total += neuron_data.get_number_of_neurons_used();
        };

        let total_length_of_byte_structure = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE +
            (number_cortical_areas * NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE) +
            (number_of_neurons_total * NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON);

        let mut output: Vec<u8> = vec![0; total_length_of_byte_structure];

        // Fill in constant size header
        output[0] = BYTE_STRUCT_ID;
        output[1] = BYTE_STRUCT_VERSION;

        let count_bytes: [u8; 2] = (number_cortical_areas as u16).to_le_bytes();
        output[2..4].copy_from_slice(&count_bytes);

        let mut header_write_index: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE;
        let mut data_write_index: u32 = header_write_index as u32 + (number_cortical_areas as u32 * NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        // fill in cortical descriptors header
        for (cortical_id, neuron_data) in mapped_data {
            // Calculate locations
            let reading_start: u32 = data_write_index;
            let reading_length: u32 = neuron_data.get_number_of_neurons_used() as u32 * NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON as u32;
            let reading_start_bytes: [u8; 4] = reading_start.to_le_bytes();
            let reading_length_bytes: [u8; 4] = reading_length.to_le_bytes();

            // Write cortical subheader
            cortical_id.write_bytes_at(&mut output[header_write_index..header_write_index + 6])?;
            output[header_write_index + 6.. header_write_index + 10].copy_from_slice(&reading_start_bytes);
            output[header_write_index + 10.. header_write_index + 14].copy_from_slice(&reading_length_bytes);

            // Write data
            neuron_data.write_data_to_bytes(&mut output[reading_start as usize .. (reading_start + reading_length) as usize])?;

            // update indexes
            data_write_index += reading_length;
            header_write_index += NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        }

        Ok(output)
    }
    
    /// Updates the internal vectors using an external function.
    /// This allows for custom in-place modifications of the neuron data vectors.
    ///
    /// # Arguments
    /// * `vectors_changer` - A function that takes mutable references to the four vectors and updates them
    ///
    /// # Returns
    /// * `Result<(), DataProcessingError>` - Success or an error if the update fails or results in the 
    ///   x y z p vectors being of different lengths by its conclusion
    pub fn update_vectors_from_external<F>(&mut self, vectors_changer: F) -> Result<(), DataProcessingError>
    where F: FnOnce(&mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>, &mut Vec<f32>) -> Result<(), DataProcessingError>
    {
        let function_result = vectors_changer(&mut self.x, &mut self.y, &mut self.c, &mut self.p);
        if function_result.is_err() {
            return function_result;
        }
        self.validate_equal_vector_lengths()
    }

    /// Returns the maximum number of neurons this structure can hold without further memory reallocation.
    ///
    /// # Returns
    /// * `usize` - Maximum neuron count capacity
    pub fn get_max_neuron_capacity_without_reallocating(&self) -> usize {
        self.x.capacity() / NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON
    }
    
    /// Expands the capacity of the vectors if the new required maximum exceeds the current maximum.
    ///
    /// # Arguments
    /// * `new_max_neuron_count` - The new maximum number of neurons required
    pub fn expand_to_new_max_count_if_required(&mut self, new_max_neuron_count: usize) {
        
        if new_max_neuron_count > self.get_max_neuron_capacity_without_reallocating() // only expand if needed
        {
            self.x = Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
            self.y = Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
            self.c = Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
            self.p = Vec::with_capacity(NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
        }
    }

    /// Clears all vectors by truncating them to zero length without deallocating its memory.
    /// This effectively resets the structure while maintaining capacity.
    pub fn reset_indexes(&mut self) {
        self.x.truncate(0);
        self.y.truncate(0);
        self.c.truncate(0);
        self.p.truncate(0);
    }

    /// Validates that all four internal vectors have the same length.
    ///
    /// # Returns
    /// * `Result<(), DataProcessingError>` - Success or an error if the vectors have different lengths
    pub fn validate_equal_vector_lengths(&self) -> Result<(), DataProcessingError> { // TODO this shouldnt be needed
        let len = self.x.len();
        if !((self.y.len() == len) && (self.x.len() == len) && (self.c.len() == len)) {
             return Err(DataProcessingError::InternalError("Internal XYCP Arrays do not have equal lengths!".into()));
        }
        Ok(())
    }

    /// Returns the current number of neurons stored in this structure.
    ///
    /// # Returns
    /// * `usize` - The number of neurons currently stored
    pub fn get_number_of_neurons_used(&self) -> usize {
        self.p.len() // all of these are of equal length
    }

    /// Writes the neuron data to a byte array in a specific interleaved format.
    ///
    /// # Arguments
    /// * `bytes_to_write_to` - The target byte array to write the data to
    ///
    /// # Returns
    /// * `Result<(), DataProcessingError>` - Success or an error if the operation fails
    fn write_data_to_bytes(&self, bytes_to_write_to: &mut [u8]) -> Result<(), DataProcessingError> {
        self.validate_equal_vector_lengths()?;
        const U32_F32_LENGTH: usize = 4;
        let number_of_neurons_to_write: usize = self.get_number_of_neurons_used();
        let number_bytes_needed = NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON * number_of_neurons_to_write;
        if bytes_to_write_to.len() != number_bytes_needed {
            return Err(DataProcessingError::InternalError("Invalid number of bytes passed to write neuronal XYCP data to!".into()))
        }

        let mut x_offset: usize = 0;
        let mut y_offset = number_of_neurons_to_write * NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON / 4; // we want to be a quarter way
        let mut c_offset = y_offset * 2; // half way
        let mut p_offset = y_offset * 3; // three quarters way

        for i in 0 .. number_of_neurons_to_write {
            let x_bytes = self.x[i].to_le_bytes();
            let y_bytes = self.y[i].to_le_bytes();
            let c_bytes = self.c[i].to_le_bytes();
            let p_bytes = self.p[i].to_le_bytes();

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
}