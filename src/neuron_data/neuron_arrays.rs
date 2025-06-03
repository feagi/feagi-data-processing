use std::collections::HashMap;
use byteorder::{ByteOrder, LittleEndian};
use crate::byte_structures::GLOBAL_HEADER_SIZE;
use crate::byte_structures::FeagiByteStructureType;
use crate::byte_structures::feagi_byte_structure::FeagiByteStructureCompatible;
use crate::cortical_data::CorticalID;
use crate::error::DataProcessingError;

/// Represents neuron data as four parallel arrays for X, Y, channel, and potential values.
/// This structure provides an efficient memory layout for serialization and processing of neuron data.
#[derive(Clone)]
pub struct NeuronXYZPArrays {
    /// X coordinates of neurons (using Cartesian coordinate system)
    x: Vec<u32>, // Remember, FEAGI is cartesian!
    /// Y coordinates of neurons
    y: Vec<u32>,
    /// Channel indices of neurons
    z: Vec<u32>,
    /// Potential/activation values of neurons
    p: Vec<f32>,
}

impl NeuronXYZPArrays {
    /// Number of bytes used to represent a single neuron in memory (going across x y z p elements)
    pub const NUMBER_BYTES_PER_NEURON: usize = 16;

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
        Ok(NeuronXYZPArrays {
            x: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            y: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            z: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            p: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
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
        NeuronXYZPArrays::new(resolution.0 * resolution.1 * resolution.2)
    }

    pub fn new_from_vectors(x: Vec<u32>, y: Vec<u32>, z: Vec<u32>, p: Vec<f32>) -> Result<Self, DataProcessingError> {
        let len = x.len();
        if len != y.len() || len != z.len() || len != p.len() {
            return Err(DataProcessingError::InvalidInputBounds("Vectors are not the same length!".into()));
        }
        Ok(NeuronXYZPArrays {
            x,
            y,
            z,
            p
        })
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
        let function_result = vectors_changer(&mut self.x, &mut self.y, &mut self.z, &mut self.p);
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
        self.x.capacity() / 4 // 4 * 4 / 4
    }

    /// Expands the capacity of the vectors if the new required maximum exceeds the current maximum.
    ///
    /// # Arguments
    /// * `new_max_neuron_count` - The new maximum number of neurons required
    pub fn expand_to_new_max_count_if_required(&mut self, new_max_neuron_count: usize) {

        if new_max_neuron_count > self.get_max_neuron_capacity_without_reallocating() // only expand if needed
        {
            self.x = Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
            self.y = Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
            self.z = Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
            self.p = Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * new_max_neuron_count);
        }
    }

    /// Clears all vectors by truncating them to zero length without deallocating its memory.
    /// This effectively resets the structure while maintaining capacity.
    pub fn reset_indexes(&mut self) {
        self.x.truncate(0);
        self.y.truncate(0);
        self.z.truncate(0);
        self.p.truncate(0);
    }

    /// Validates that all four internal vectors have the same length.
    ///
    /// # Returns
    /// * `Result<(), DataProcessingError>` - Success or an error if the vectors have different lengths
    pub fn validate_equal_vector_lengths(&self) -> Result<(), DataProcessingError> { // TODO make internal
        let len = self.x.len();
        if !((self.y.len() == len) && (self.x.len() == len) && (self.z.len() == len)) {
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

    pub fn borrow_xyzp_vectors(&self) -> (&Vec<u32>, &Vec<u32>, &Vec<u32>, &Vec<f32>) {
        (&self.x, &self.y, &self.z, &self.p)
    }

    pub fn write_neural_data_to_bytes(&self, bytes_to_write_to: &mut [u8]) -> Result<(), DataProcessingError> {
        const U32_F32_LENGTH: usize = 4;
        let number_of_neurons_to_write: usize = self.get_number_of_neurons_used();
        let number_bytes_needed = NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * number_of_neurons_to_write;
        if bytes_to_write_to.len() != number_bytes_needed {
            return Err(DataProcessingError::InvalidByteStructure(format!("Need exactly {} bytes to write xyzp neuron data, but given a space of {} bytes!", bytes_to_write_to.len(), number_bytes_needed).into()))
        }
        let mut x_offset: usize = 0;
        let mut y_offset = number_of_neurons_to_write * NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON / 4; // we want to be a quarter way
        let mut z_offset = y_offset * 2; // half way
        let mut p_offset = y_offset * 3; // three quarters way

        for i in 0 .. number_of_neurons_to_write {
            LittleEndian::write_u32(&mut bytes_to_write_to[x_offset .. x_offset + U32_F32_LENGTH], self.x[i]);
            LittleEndian::write_u32(&mut bytes_to_write_to[y_offset .. y_offset + U32_F32_LENGTH], self.y[i]);
            LittleEndian::write_u32(&mut bytes_to_write_to[z_offset .. z_offset + U32_F32_LENGTH], self.z[i]);
            LittleEndian::write_f32(&mut bytes_to_write_to[p_offset .. p_offset + U32_F32_LENGTH], self.p[i]);

            x_offset += U32_F32_LENGTH;
            y_offset += U32_F32_LENGTH;
            z_offset += U32_F32_LENGTH;
            p_offset += U32_F32_LENGTH;
        };

        Ok(())
    }
}