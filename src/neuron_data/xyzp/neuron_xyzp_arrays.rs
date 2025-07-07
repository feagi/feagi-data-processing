use std::ops::RangeInclusive;
use ndarray::Array1;
use byteorder::{ByteOrder, LittleEndian};
use crate::error::{NeuronError, FeagiBytesError, FeagiDataProcessingError};
use crate::neuron_data::xyzp::NeuronXYZP;

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

    /// Creates a new NeuronXYZPArrays instance with capacity for the specified maximum number of neurons.
    ///
    /// # Arguments
    /// * `maximum_number_of_neurons_possibly_needed` - The maximum number of neurons this structure should be able to hold
    ///
    /// # Returns
    /// * `Result<Self, NeuronError>` - A new instance or an error if the input is invalid
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> Result<Self, NeuronError> {
        if maximum_number_of_neurons_possibly_needed == 0 {
            return Err(NeuronError::UnableToGenerateNeuronData("Given number of neurons possible must be greater than 0!".into()));
        };
        Ok(NeuronXYZPArrays {
            x: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            y: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            z: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            p: Vec::with_capacity(NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
        })
    }

    /// Creates a new NeuronXYZPArrays from a 3D resolution tuple.
    ///
    /// # Arguments
    /// * `resolution` - A tuple representing the 3D dimensions (neuron count) in the x y z directions 
    ///
    /// # Returns
    /// * `Result<Self, NeuronError>` - A new instance with capacity for all neurons in the 3D space
    pub fn new_from_resolution(resolution: (usize, usize, usize)) -> Result<Self, NeuronError> {
        NeuronXYZPArrays::new(resolution.0 * resolution.1 * resolution.2)
    }

    /// Creates a new NeuronXYZPArrays instance from four separate vectors of equal length.
    ///
    /// # Arguments
    /// * `x` - Vector of X coordinates
    /// * `y` - Vector of Y coordinates
    /// * `z` - Vector of Z coordinates (channel indices)
    /// * `p` - Vector of potential/activation values
    ///
    /// # Returns
    /// * `Result<Self, NeuronError>` - A new instance or an error if the vectors have different lengths
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPArrays;
    /// 
    /// let x = vec![1, 2, 3];
    /// let y = vec![4, 5, 6];
    /// let z = vec![7, 8, 9];
    /// let p = vec![0.1, 0.2, 0.3];
    /// 
    /// let arrays = NeuronXYZPArrays::new_from_vectors(x, y, z, p).unwrap();
    /// assert_eq!(arrays.get_number_of_neurons_used(), 3);
    /// ```
    pub fn new_from_vectors(x: Vec<u32>, y: Vec<u32>, z: Vec<u32>, p: Vec<f32>) -> Result<Self, NeuronError> {
        let len = x.len();
        if len != y.len() || len != z.len() || len != p.len() {
            return Err(NeuronError::UnableToGenerateNeuronData("Input vectors must be the same length to generate XYZP neuron data!!".into()));
        }
        Ok(NeuronXYZPArrays {
            x,
            y,
            z,
            p
        })
    }

    /// Creates a new NeuronXYZPArrays instance from four ndarray Array1 instances of equal length.
    ///
    /// # Arguments
    /// * `x_nd` - Array1 of X coordinates
    /// * `y_nd` - Array1 of Y coordinates
    /// * `z_nd` - Array1 of Z coordinates (channel indices)
    /// * `p_nd` - Array1 of potential/activation values
    ///
    /// # Returns
    /// * `Result<Self, NeuronError>` - A new instance or an error if the arrays have different lengths
    ///
    /// # Examples
    /// ```
    /// use ndarray::Array1;
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPArrays;
    /// 
    /// let x_nd = Array1::from_vec(vec![1, 2, 3]);
    /// let y_nd = Array1::from_vec(vec![4, 5, 6]);
    /// let z_nd = Array1::from_vec(vec![7, 8, 9]);
    /// let p_nd = Array1::from_vec(vec![0.1, 0.2, 0.3]);
    /// 
    /// let arrays = NeuronXYZPArrays::new_from_ndarrays(x_nd, y_nd, z_nd, p_nd).unwrap();
    /// assert_eq!(arrays.get_number_of_neurons_used(), 3);
    /// ```
    pub fn new_from_ndarrays(x_nd: Array1<u32>, y_nd: Array1<u32>, z_nd: Array1<u32>, p_nd: Array1<f32>) -> Result<Self, NeuronError> {
        let len = x_nd.len();
        if len != y_nd.len() || len != z_nd.len() || len != p_nd.len() {
            return Err(NeuronError::UnableToGenerateNeuronData("ND Arrays must be the same length to generate XYZP neuron data!".into()));
        }
        Ok(NeuronXYZPArrays{ x: x_nd.to_vec(), y: y_nd.to_vec(), z: z_nd.to_vec(), p: p_nd.to_vec()})
    }

    /// Updates the internal vectors using an external function.
    /// This allows for custom in-place modifications of the neuron data vectors.
    ///
    /// # Arguments
    /// * `vectors_changer` - A function that takes mutable references to the four vectors and updates them
    ///
    /// # Returns
    /// * `Result<(), NeuronError>` - Success or an error if the update fails or results in the 
    ///   x y z p vectors being of different lengths by its conclusion
    pub fn update_vectors_from_external<F>(&mut self, vectors_changer: F) -> Result<(), FeagiDataProcessingError>
    where F: FnOnce(&mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>, &mut Vec<f32>) -> Result<(), FeagiDataProcessingError>
    {
        vectors_changer(&mut self.x, &mut self.y, &mut self.z, &mut self.p)?;
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
    
    /// Adds a single neuron to the arrays.
    ///
    /// # Arguments
    /// * `neuron` - The NeuronXYZP instance to add
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(1).unwrap();
    /// let neuron = NeuronXYZP::new(1, 2, 3, 0.5);
    /// arrays.add_neuron(&neuron);
    /// assert_eq!(arrays.get_number_of_neurons_used(), 1);
    /// ```
    pub fn add_neuron(&mut self, neuron: &NeuronXYZP) {
        self.x.push(neuron.x);
        self.y.push(neuron.y);
        self.z.push(neuron.z);
        self.p.push(neuron.p);
    }
    
    /// Creates a vector of NeuronXYZP instances from the current arrays.
    ///
    /// # Returns
    /// * `Vec<NeuronXYZP>` - A vector containing all neurons as individual NeuronXYZP instances
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(2).unwrap();
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.add_neuron(&NeuronXYZP::new(4, 5, 6, 0.7));
    ///
    /// let neurons = arrays.copy_as_neuron_xyzp_vec();
    /// assert_eq!(neurons.len(), 2);
    /// assert_eq!(neurons[0].x, 1);
    /// assert_eq!(neurons[1].p, 0.7);
    /// ```
    pub fn copy_as_neuron_xyzp_vec(&self) -> Vec<NeuronXYZP> {
        let mut output: Vec<NeuronXYZP> = Vec::new();
        for i in 0..self.x.len() {
            output.push(NeuronXYZP::new(self.x[i], self.y[i], self.z[i], self.p[i]));
        };
        return output;
    }
    
    /// Converts the current arrays into a tuple of ndarray Array1 instances.
    ///
    /// # Returns
    /// * `(Array1<u32>, Array1<u32>, Array1<u32>, Array1<f32>)` - A tuple containing the four arrays
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(2).unwrap();
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.add_neuron(&NeuronXYZP::new(4, 5, 6, 0.7));
    ///
    /// let (x, y, z, p) = arrays.copy_as_tuple_of_nd_arrays();
    /// assert_eq!(x[0], 1);
    /// assert_eq!(y[1], 5);
    /// assert_eq!(z[0], 3);
    /// assert_eq!(p[1], 0.7);
    /// ```
    pub fn copy_as_tuple_of_nd_arrays(&self) -> (Array1<u32>, Array1<u32>, Array1<u32>, Array1<f32>) {
        (
            Array1::from_vec(self.x.clone()),
            Array1::from_vec(self.y.clone()),
            Array1::from_vec(self.z.clone()),
            Array1::from_vec(self.p.clone())
        )
    }
    
    /// Returns an iterator over all neurons in the arrays.
    ///
    /// # Returns
    /// * `impl Iterator<Item=NeuronXYZP> + '_` - An iterator yielding NeuronXYZP instances
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(2).unwrap();
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.add_neuron(&NeuronXYZP::new(4, 5, 6, 0.7));
    ///
    /// let mut iter = arrays.iter();
    /// let first = iter.next().unwrap();
    /// assert_eq!(first.x, 1);
    /// assert_eq!(first.p, 0.5);
    ///
    /// let second = iter.next().unwrap();
    /// assert_eq!(second.y, 5);
    /// assert_eq!(second.z, 6);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item=NeuronXYZP> + '_ {
        self.x.iter()
            .zip(&self.y)
            .zip(&self.z)
            .zip(&self.p)
            .map(|(((x,y),z),p)| NeuronXYZP {
            x: *x,
            y: *y,
            z: *z,
            p: *p
        })
    }

    /// Validates that all four internal vectors have the same length.
    ///
    /// # Returns
    /// * `Result<(), NeuronError>` - Success or an error if the vectors have different lengths
    pub fn validate_equal_vector_lengths(&self) -> Result<(), FeagiDataProcessingError> { // TODO make internal
        let len = self.x.len();
        if !((self.y.len() == len) && (self.x.len() == len) && (self.z.len() == len)) {
            return Err(NeuronError::UnableToParseFromNeuronData("Internal XYCP Arrays do not have equal lengths!".into()).into());
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
    
    /// Checks if no neurons are in this structure.
    ///
    /// # Returns
    /// * `bool` - True if there are no neurons stored, false otherwise
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(1).unwrap();
    /// assert!(arrays.is_empty());
    ///
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// assert!(!arrays.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    /// Returns references to the internal vectors.
    ///
    /// # Returns
    /// * `(&Vec<u32>, &Vec<u32>, &Vec<u32>, &Vec<f32>)` - References to the x, y, z, and p vectors
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(1).unwrap();
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    ///
    /// let (x, y, z, p) = arrays.borrow_xyzp_vectors();
    /// assert_eq!(x[0], 1);
    /// assert_eq!(y[0], 2);
    /// assert_eq!(z[0], 3);
    /// assert_eq!(p[0], 0.5);
    /// ```
    pub fn borrow_xyzp_vectors(&self) -> (&Vec<u32>, &Vec<u32>, &Vec<u32>, &Vec<f32>) {
        (&self.x, &self.y, &self.z, &self.p)
    }

    /// Writes the neural data to a byte buffer.
    ///
    /// The data is written in the following order: all x values, all y values, all z values, all p values.
    /// Each value is written using little-endian byte order.
    ///
    /// # Arguments
    /// * `bytes_to_write_to` - The byte buffer to write the data to
    ///
    /// # Returns
    /// * `Result<(), NeuronError>` - Success or an error if the buffer size is incorrect
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(1).unwrap();
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    ///
    /// let mut buffer = vec![0u8; NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON];
    /// arrays.write_neural_data_to_bytes(&mut buffer).unwrap();
    /// ```
    pub fn write_neural_data_to_bytes(&self, bytes_to_write_to: &mut [u8]) -> Result<(), FeagiBytesError> {
        const U32_F32_LENGTH: usize = 4;
        let number_of_neurons_to_write: usize = self.get_number_of_neurons_used();
        let number_bytes_needed = NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON * number_of_neurons_to_write;
        if bytes_to_write_to.len() != number_bytes_needed {
            return Err(FeagiBytesError::UnableToSerializeBytes(format!("Need exactly {} bytes to write xyzp neuron data, but given a space of {} bytes!", bytes_to_write_to.len(), number_bytes_needed).into()))
        }
        let mut x_offset: usize = 0;
        let mut y_offset = number_of_neurons_to_write * U32_F32_LENGTH; // quarter way through the total bytes
        let mut z_offset = number_of_neurons_to_write * U32_F32_LENGTH * 2; // half way through the total bytes
        let mut p_offset = number_of_neurons_to_write * U32_F32_LENGTH * 3; // three quarters way through the total bytes

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
    
    /// Creates a new NeuronXYZPArrays from filtering neurons based on their locations.
    ///
    /// # Arguments
    /// * `x_range` - Range of valid X coordinates
    /// * `y_range` - Range of valid Y coordinates
    /// * `z_range` - Range of valid Z coordinates
    ///
    /// # Returns
    /// * `Result<NeuronXYZPArrays, NeuronError>` - A new instance containing only neurons within the specified ranges
    ///
    /// # Examples
    /// ```
    /// use std::ops::RangeInclusive;
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::new(3).unwrap();
    /// arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.add_neuron(&NeuronXYZP::new(4, 5, 6, 0.7));
    /// arrays.add_neuron(&NeuronXYZP::new(7, 8, 9, 0.9));
    ///
    /// let filtered = arrays.filter_neurons_by_location_bounds(
    ///     RangeInclusive::new(1, 4),
    ///     RangeInclusive::new(2, 5),
    ///     RangeInclusive::new(3, 6)
    /// ).unwrap();
    ///
    /// assert_eq!(filtered.get_number_of_neurons_used(), 2);
    /// ```
    pub fn filter_neurons_by_location_bounds(&self, x_range: RangeInclusive<u32>, y_range: RangeInclusive<u32>, z_range: RangeInclusive<u32>) -> Result<NeuronXYZPArrays, NeuronError> {
        let mut xv: Vec<u32> = Vec::new();
        let mut yv: Vec<u32> = Vec::new();
        let mut zv: Vec<u32> = Vec::new();
        let mut pv: Vec<f32> = Vec::new();
        
        // TODO Could this be optimized at all?
        for (&x, (&y, (&z, &p))) in self.x.iter()
            .zip(self.y.iter()
                .zip(self.z.iter()
                    .zip(self.p.iter()))) {
            if x_range.contains(&x)
                && y_range.contains(&y)
                && z_range.contains(&z)
            {
                xv.push(x);
                yv.push(y);
                zv.push(z);
                pv.push(p);
            }
        };
        
        NeuronXYZPArrays::new_from_vectors(xv, yv, zv, pv)
    }
}