//! Efficient array-based neuron data storage and processing.
//!
//! This module provides the `NeuronXYZPArrays` structure, which stores neuron data
//! as parallel arrays for efficient batch processing, serialization, and memory usage.
//! The array-based approach enables vectorized operations and optimal memory layouts
//! for high-performance neural network simulation.

use std::ops::{RangeInclusive};
use ndarray::Array1;
use byteorder::{ByteOrder, LittleEndian};
use crate::error::{NeuronError, FeagiBytesError, FeagiDataProcessingError, IODataError};
use crate::neuron_data::xyzp::NeuronXYZP;

/// Efficient parallel array storage for neuron XYZP data.
///
/// `NeuronXYZPArrays` stores neuron data as four separate parallel arrays (X, Y, Z, P)
/// rather than an array of XYZP structures. This layout is useful in context of multiprocessing.
///
/// # Memory Layout
/// ```text
/// X: [x₀, x₁, x₂, ..., xₙ]
/// Y: [y₀, y₁, y₂, ..., yₙ]
/// Z: [z₀, z₁, z₂, ..., zₙ]
/// P: [p₀, p₁, p₂, ..., pₙ]
/// ```
///
/// Each neuron's complete coordinate is at the same index across all arrays.
///
/// # Capacity Management
/// The structure pre-allocates capacity to avoid frequent reallocations during
/// neural network simulation. Use `new()` with an estimated maximum neuron count
/// for optimal performance.
///
/// # Usage Examples
///
/// ## Basic Operations
/// ```rust
/// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
///
/// // Create with capacity for 1000 neurons
/// let mut arrays = NeuronXYZPArrays::with_capacity(1000);
///
/// // Add individual neurons
/// arrays.push(&NeuronXYZP::new(10, 5, 2, 0.8));
/// arrays.push(&NeuronXYZP::new(11, 6, 3, 0.6));
///
/// // Access neuron count and data
/// assert_eq!(arrays.len(), 2);
/// ```
///
/// # Thread Safety
/// `NeuronXYZPArrays` is not inherently thread-safe. Use external synchronization
/// (e.g., `Mutex`, `RwLock`) when accessing from multiple threads.
///
/// # Binary Serialization
/// The structure supports efficient binary serialization with a fixed format:
/// - Each neuron uses exactly 16 bytes (4 bytes each for X, Y, Z, P)
/// - Little-endian byte order for cross-platform compatibility
/// - Compact format optimized for network transmission
#[derive(Clone,Debug)]
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

impl std::fmt::Display for NeuronXYZPArrays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!("NeuronXYZPArrays(X: {:?}, Y: {:?}, Z: {:?}, P: {:?})", self.x, self.y, self.z, self.p);
        write!(f, "{}", s)
    }
}

impl NeuronXYZPArrays {
    /// Creates a new empty NeuronXYZPArrays instance.
    ///
    /// # Returns
    /// * `Self` - A new empty instance with no allocated capacity
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPArrays;
    ///
    /// let arrays = NeuronXYZPArrays::new();
    /// assert_eq!(arrays.len(), 0);
    /// assert!(arrays.is_empty());
    /// ```
    pub fn new() -> Self {
        NeuronXYZPArrays {
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
            p: Vec::new(),
        }
    }

    /// Creates a new NeuronXYZPArrays from a 3D resolution tuple.
    ///
    /// # Arguments
    /// * `resolution` - A tuple representing the 3D dimensions (neuron count) in the x y z directions 
    ///
    /// # Returns
    /// * `Self` - A new instance with capacity for all neurons in the 3D space
    pub fn new_from_resolution(resolution: (usize, usize, usize)) -> Self {
        NeuronXYZPArrays::with_capacity(resolution.0 * resolution.1 * resolution.2)
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
    /// assert_eq!(arrays.len(), 3);
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
    /// assert_eq!(arrays.len(), 3);
    /// ```
    pub fn new_from_ndarrays(x_nd: Array1<u32>, y_nd: Array1<u32>, z_nd: Array1<u32>, p_nd: Array1<f32>) -> Result<Self, NeuronError> {
        let len = x_nd.len();
        if len != y_nd.len() || len != z_nd.len() || len != p_nd.len() {
            return Err(NeuronError::UnableToGenerateNeuronData("ND Arrays must be the same length to generate XYZP neuron data!".into()));
        }
        Ok(NeuronXYZPArrays{ x: x_nd.to_vec(), y: y_nd.to_vec(), z: z_nd.to_vec(), p: p_nd.to_vec()})
    }
    
    //region Array-Like Implementations
    
    /// Creates a new NeuronXYZPArrays instance with capacity for the specified maximum number of neurons.
    ///
    /// # Arguments
    /// * `number_of_neurons_initial` - The number of neurons to allocate space for
    ///
    /// # Returns
    /// * `Self` - A new instance
    pub fn with_capacity(number_of_neurons_initial: usize) -> Self {
        NeuronXYZPArrays {
            x: Vec::with_capacity(number_of_neurons_initial),
            y: Vec::with_capacity(number_of_neurons_initial),
            z: Vec::with_capacity(number_of_neurons_initial),
            p: Vec::with_capacity(number_of_neurons_initial),
        }
    }
    
    /// Returns the current capacity of the internal vectors.
    ///
    /// # Returns
    /// * `usize` - The maximum number of neurons that can be stored without reallocation
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPArrays;
    ///
    /// let arrays = NeuronXYZPArrays::with_capacity(100);
    /// assert_eq!(arrays.capacity(), 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.x.capacity() // all are the same
    }
    
    /// Returns the number of additional neurons that can be stored without reallocation.
    ///
    /// # Returns
    /// * `usize` - The difference between capacity and current length
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(10);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// assert_eq!(arrays.spare_capacity(), 9);
    /// ```
    pub fn spare_capacity(&self) -> usize {
        self.x.capacity() - self.x.len()
    }

    /// Returns the current number of neurons stored in this structure.
    ///
    /// # Returns
    /// * `usize` - The number of neurons currently stored
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// assert_eq!(arrays.len(), 0);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// assert_eq!(arrays.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.p.len() // all of these are of equal length
    }
    
    /// Shrinks the capacity of all internal vectors to match their current length.
    ///
    /// This reduces memory usage by deallocating unused capacity.
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(100);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.shrink_to_fit();
    /// assert_eq!(arrays.capacity(), 1);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.x.shrink_to_fit();
        self.y.shrink_to_fit();
        self.z.shrink_to_fit();
        self.p.shrink_to_fit();
    }
    
    /// Ensures the vectors have at least the specified total capacity.
    ///
    /// If the current capacity is already sufficient, this function does nothing.
    /// Otherwise, it reserves additional space to reach the target capacity.
    ///
    /// # Arguments
    /// * `number_of_neurons_total` - The minimum total capacity required
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPArrays;
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(10);
    /// arrays.ensure_capacity(50);
    /// assert!(arrays.capacity() >= 50);
    /// ```
    pub fn ensure_capacity(&mut self, number_of_neurons_total: usize) {
        if self.capacity() >= number_of_neurons_total {
            return;
        }
        self.reserve(number_of_neurons_total - self.capacity());
    }
    
    /// Reserves capacity for at least the specified number of additional neurons.
    ///
    /// The actual capacity reserved may be greater than requested to optimize
    /// for future insertions. This operation affects all four internal vectors.
    ///
    /// # Arguments
    /// * `additional_neuron_count` - The number of additional neurons to reserve space for
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPArrays;
    ///
    /// let mut arrays = NeuronXYZPArrays::new();
    /// arrays.reserve(100);
    /// assert!(arrays.capacity() >= 100);
    /// ```
    pub fn reserve(&mut self, additional_neuron_count: usize) {
        self.x.reserve(additional_neuron_count);
        self.y.reserve(additional_neuron_count);
        self.z.reserve(additional_neuron_count);
        self.p.reserve(additional_neuron_count);
    }

    /// Adds a single neuron to the end of the arrays.
    ///
    /// # Arguments
    /// * `neuron` - The NeuronXYZP instance to add
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// let neuron = NeuronXYZP::new(1, 2, 3, 0.5);
    /// arrays.push(&neuron);
    /// assert_eq!(arrays.len(), 1);
    /// ```
    pub fn push(&mut self, neuron: &NeuronXYZP) {
        self.x.push(neuron.x);
        self.y.push(neuron.y);
        self.z.push(neuron.z);
        self.p.push(neuron.p);
    }

    /// Gets a neuron at the specified index.
    ///
    /// # Arguments
    /// * `index` - The index of the neuron to retrieve
    ///
    /// # Returns
    /// * `Result<NeuronXYZP, FeagiDataProcessingError>` - The neuron at the index or an error if out of bounds
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// let neuron = arrays.get(0).unwrap();
    /// assert_eq!(neuron.x, 1);
    /// ```
    pub fn get(&mut self, index: usize) -> Result<NeuronXYZP, FeagiDataProcessingError> {
        if index >= self.len()  {
            return Err(IODataError::InvalidParameters(format!("Given index {} is exceeds NeuronXYZPArray length of {}!", index, self.len())).into())
        }
        let x = self.x[index];
        let y = self.y[index];
        let z = self.z[index];
        let p = self.p[index];
        Ok(NeuronXYZP{x, y, z, p})
    }
    
    /// Removes and returns the last neuron from the arrays.
    ///
    /// # Returns
    /// * `Option<NeuronXYZP>` - The last neuron if the arrays are not empty, `None` otherwise
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// let neuron = arrays.pop().unwrap();
    /// assert_eq!(neuron.x, 1);
    /// assert!(arrays.is_empty());
    /// ```
    pub fn pop(&mut self) -> Option<NeuronXYZP> {
        let x = self.x.pop();
        let y = self.y.pop();
        let z = self.z.pop();
        let p = self.p.pop();
        match x {
            Some(x) => {
                Some(NeuronXYZP {
                    x,
                    y: y.unwrap(),
                    z: z.unwrap(),
                    p: p.unwrap()
                })
            }
            None => {None}
        }
    }

    /// Clears all vectors by truncating them to zero length without deallocating its memory.
    /// This effectively resets the structure while maintaining capacity.
    pub fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
        self.z.clear();
        self.p.clear();
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// assert!(arrays.is_empty());
    ///
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// assert!(!arrays.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(2);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.push(&NeuronXYZP::new(4, 5, 6, 0.7));
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
    
    /// Returns an iterator over all neurons with their indices.
    ///
    /// # Returns
    /// * `impl Iterator<Item=(usize, NeuronXYZP)> + '_` - An iterator yielding (index, neuron) pairs
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(2);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.push(&NeuronXYZP::new(4, 5, 6, 0.7));
    ///
    /// for (index, neuron) in arrays.enumerate() {
    ///     println!("Neuron {} at position {}", neuron.x, index);
    /// }
    /// ```
    pub fn enumerate(&self) -> impl Iterator<Item=(usize, NeuronXYZP)> + '_ {
        self.x.iter().enumerate()
            .zip(&self.y)
            .zip(&self.z)
            .zip(&self.p)
            .map(|(((x,y),z),p)| 
                (x.0,
                NeuronXYZP {
                    x: *x.1,
                    y: *y,
                    z: *z,
                    p: *p
            }))
    }
    
    //endregion
    
    /// Updates the internal vectors using an external function.
    /// This allows for custom in-place modifications of the neuron data vectors.
    ///
    /// # Arguments
    /// * `vectors_changer` - A function that takes mutable references to the four vectors and updates them
    ///
    /// # Returns
    /// * `Result<(), NeuronError>` - Success or an error if the update fails or results in the 
    ///   x y z p vectors being of different lengths by its conclusion
    pub(crate) fn update_vectors_from_external<F>(&mut self, vectors_changer: F) -> Result<(), FeagiDataProcessingError>
    where F: FnOnce(&mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>, &mut Vec<f32>) -> Result<(), FeagiDataProcessingError>
    {
        vectors_changer(&mut self.x, &mut self.y, &mut self.z, &mut self.p)?;
        self.validate_equal_vector_lengths()
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(2);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.push(&NeuronXYZP::new(4, 5, 6, 0.7));
    ///
    /// let neurons = arrays.copy_as_neuron_xyzp_vec();
    /// assert_eq!(neurons.len(), 2);
    /// assert_eq!(neurons[0].x, 1);
    /// assert_eq!(neurons[1].p, 0.7);
    /// ```
    pub fn copy_as_neuron_xyzp_vec(&self) -> Vec<NeuronXYZP> {
        let mut output: Vec<NeuronXYZP> = Vec::with_capacity(self.len());
        for i in 0..self.x.len() {
            output.push(NeuronXYZP::new(self.x[i], self.y[i], self.z[i], self.p[i]));
        };
        output
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(2);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.push(&NeuronXYZP::new(4, 5, 6, 0.7));
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
    
    /// Returns the total size in bytes required to store all neurons.
    ///
    /// This calculates the number of bytes needed for binary serialization
    /// of the current neuron data.
    ///
    /// # Returns
    /// * `usize` - Total bytes required (number of neurons × 16 bytes per neuron)
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZP};
    ///
    /// let mut arrays = NeuronXYZPArrays::with_capacity(2);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.push(&NeuronXYZP::new(4, 5, 6, 0.7));
    /// assert_eq!(arrays.get_size_in_number_of_bytes(), 32); // 2 neurons × 16 bytes
    /// ```
    pub fn get_size_in_number_of_bytes(&self) -> usize {
        self.len() * NeuronXYZP::NUMBER_BYTES_PER_NEURON
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(1);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    ///
    /// let mut buffer = vec![0u8; NeuronXYZP::NUMBER_BYTES_PER_NEURON];
    /// arrays.write_neural_data_to_bytes(&mut buffer).unwrap();
    /// ```
    pub fn write_neural_data_to_bytes(&self, bytes_to_write_to: &mut [u8]) -> Result<(), FeagiBytesError> {
        const U32_F32_LENGTH: usize = 4;
        let number_of_neurons_to_write: usize = self.len();
        let number_bytes_needed = self.get_size_in_number_of_bytes();
        if bytes_to_write_to.len() != number_bytes_needed {
            return Err(FeagiBytesError::UnableToSerializeBytes(format!("Need exactly {} bytes to write xyzp neuron data, but given a space of {} bytes!", bytes_to_write_to.len(), number_bytes_needed).into()))
        }
        let mut x_offset: usize = 0;
        let mut y_offset = number_of_neurons_to_write * U32_F32_LENGTH; // quarter way through the total bytes
        let mut z_offset = number_of_neurons_to_write * U32_F32_LENGTH * 2; // halfway through the total bytes
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
    /// let mut arrays = NeuronXYZPArrays::with_capacity(3);
    /// arrays.push(&NeuronXYZP::new(1, 2, 3, 0.5));
    /// arrays.push(&NeuronXYZP::new(4, 5, 6, 0.7));
    /// arrays.push(&NeuronXYZP::new(7, 8, 9, 0.9));
    ///
    /// let filtered = arrays.filter_neurons_by_location_bounds(
    ///     RangeInclusive::new(1, 4),
    ///     RangeInclusive::new(2, 5),
    ///     RangeInclusive::new(3, 6)
    /// ).unwrap();
    ///
    /// assert_eq!(filtered.len(), 2);
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


    /// Validates that all four internal vectors have the same length. This must never fail.
    ///
    /// # Returns
    /// * `Result<(), NeuronError>` - Success or an error if the vectors have different lengths
    fn validate_equal_vector_lengths(&self) -> Result<(), FeagiDataProcessingError> {
        let len = self.x.len();
        if !((self.y.len() == len) && (self.x.len() == len) && (self.z.len() == len)) {
            return Err(FeagiDataProcessingError::InternalError("Internal XYCP Arrays do not have equal lengths!".into()).into());
        }
        Ok(())
    }
    
}