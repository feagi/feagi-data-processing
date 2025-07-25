//! Cortical area mapped neuron data collections.
//!
//! This module provides [`CorticalMappedXYZPNeuronData`], the primary data structure
//! for managing collections of neurons organized by cortical areas in the FEAGI system.
//! It supports efficient storage, retrieval, and serialization of neuron data across
//! multiple brain regions.
//!
//! # Overview
//!
//! The [`CorticalMappedXYZPNeuronData`] structure maps cortical area identifiers to
//! collections of neurons represented as [`NeuronXYZPArrays`]. This organization
//! mirrors the biological brain's structure where neurons are grouped into distinct
//! cortical areas with specific functions.
//!
//! # Key Features
//!
//! - **Cortical Organization**: Neurons grouped by cortical areas for biological accuracy
//! - **Efficient Storage**: Uses optimized array structures for high-performance processing
//! - **Network Serialization**: Built-in support for binary serialization/deserialization
//! - **Dynamic Management**: Runtime insertion, removal, and modification of neuron collections
//!
//! # Binary Serialization
//!
//! The structure supports efficient binary serialization for network transmission
//! and persistent storage, implementing the [`FeagiByteStructureCompatible`] trait.
//!
//! # Performance Considerations
//!
//! - Use `new_with_capacity()` when the number of cortical areas is known
//! - Consider using `ensure_clear_and_borrow_mut()` for repeated data updates
//! - The binary format is optimized for minimal network overhead

use std::collections::HashMap;
use byteorder::{ByteOrder, LittleEndian};
use crate::error::{FeagiBytesError, FeagiDataProcessingError};
use crate::io_processing::byte_structures::{FeagiByteStructureType, FeagiByteStructure, FeagiByteStructureCompatible};
use crate::genomic_structures::{CorticalID};
use crate::neuron_data::xyzp::NeuronXYZPArrays;

/// Collection of neuron data organized by cortical areas.
///
/// This structure provides the main interface for managing neuron data across multiple
/// cortical areas in the FEAGI system. Each cortical area is identified by a unique
/// [`CorticalID`] and contains a collection of neurons represented as [`NeuronXYZPArrays`].
///
/// # Structure
///
/// The data is organized as a hash map where:
/// - **Key**: [`CorticalID`] - Unique identifier for each cortical area
/// - **Value**: [`NeuronXYZPArrays`] - Collection of neurons in that cortical area
///
/// # Binary Format
///
/// The structure supports binary serialization with the following format:
/// - Global header (4 bytes): Type and version information
/// - Cortical count (2 bytes): Number of cortical areas
/// - Cortical headers (14 bytes each): Area metadata and data pointers
/// - Neuron data (variable): Raw neuron XYZP data for all areas
#[derive(Clone)]
pub struct CorticalMappedXYZPNeuronData {
    /// Hash map storing neuron collections for each cortical area.
    ///
    /// The key is a unique cortical area identifier, and the value contains
    /// all neurons belonging to that cortical area.
    pub mappings: HashMap<CorticalID, NeuronXYZPArrays>,
}

impl FeagiByteStructureCompatible for CorticalMappedXYZPNeuronData {
    fn get_type(&self) -> FeagiByteStructureType { Self::BYTE_STRUCT_TYPE }
    fn get_version(&self) -> u8 { Self::BYTE_STRUCT_VERSION }
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, FeagiDataProcessingError> {
        
        if self.mappings.len() == 0 {
            return Err(FeagiBytesError::UnableToSerializeBytes("Cannot generate a byte structure export with an empty cortical mappings object!".into()).into())
        }
        
        let num_bytes_needed: usize = self.max_number_bytes_needed();
        if slice.len() < num_bytes_needed {
            return Err(FeagiBytesError::UnableToSerializeBytes(format!("Not enough space given to store neuron XYZP data! Need {} bytes but given {}!", num_bytes_needed, slice.len())).into());
        }

        slice[0] = self.get_type() as u8;
        slice[1] = self.get_version();

        let number_cortical_areas: usize = self.mappings.len();
        LittleEndian::write_u16(&mut slice[2..4], number_cortical_areas as u16);

        let mut subheader_write_index: usize = FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::CORTICAL_COUNT_HEADER_SIZE;
        let mut neuron_data_write_index: u32 = subheader_write_index as u32 + (number_cortical_areas as u32 * Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        for (cortical_id, neuron_data) in &self.mappings {
            // Write cortical subheader
            let write_target = &mut slice[subheader_write_index .. subheader_write_index + CorticalID::CORTICAL_ID_LENGTH];
            let write_target = write_target.try_into().unwrap();
            cortical_id.write_bytes_at(write_target)?;
            let reading_start: u32 = neuron_data_write_index;
            let reading_length: u32 = neuron_data.get_number_of_neurons_used() as u32 * NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON as u32;
            LittleEndian::write_u32(&mut slice[subheader_write_index + 6 .. subheader_write_index + 10], reading_start);
            LittleEndian::write_u32(&mut slice[subheader_write_index + 10 .. subheader_write_index + 14], reading_length);

            // write neuron data
            neuron_data.write_neural_data_to_bytes(&mut slice[reading_start as usize .. (reading_start + reading_length) as usize])?;

            // update indexes
            neuron_data_write_index += reading_length;
            subheader_write_index += Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        };
        let wasted_space = slice.len() - num_bytes_needed;
        Ok(wasted_space)
    }

    fn max_number_bytes_needed(&self) -> usize {
        let mut number_neurons: usize = 0;
        for neuron_set in self.mappings.values() {
            number_neurons += neuron_set.get_number_of_neurons_used();
        }
        let bytes_needed_for_neurons = number_neurons * NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON;
        FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::CORTICAL_COUNT_HEADER_SIZE + 
            (self.get_number_contained_areas() * Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as usize) +
            bytes_needed_for_neurons
    }

    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, FeagiDataProcessingError> {
        FeagiByteStructure::verify_matching_structure_type_and_version(&feagi_byte_structure,
                                                   Self::BYTE_STRUCT_TYPE,
                                                   Self::BYTE_STRUCT_VERSION)?;
        
        let bytes = feagi_byte_structure.borrow_data_as_slice();
        let number_cortical_areas: u16 = LittleEndian::read_u16(&bytes[2..4]);

        let min_array_length_with_cortical_headers: usize = FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES +  Self::CORTICAL_COUNT_HEADER_SIZE +
            (Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE * number_cortical_areas as usize);

        if bytes.len() < min_array_length_with_cortical_headers {
            return Err(FeagiBytesError::UnableToSerializeBytes(format!("Byte structure for NeuronCategoricalXYZPV1 needs a length of {} to fit just the cortical details header, but is a length of {}",
                                                                         min_array_length_with_cortical_headers, bytes.len())).into());
        }

        let number_cortical_areas: usize = number_cortical_areas as usize;
        let mut output: CorticalMappedXYZPNeuronData = CorticalMappedXYZPNeuronData::new_with_capacity(number_cortical_areas);
        
        let mut reading_header_index: usize = FeagiByteStructure::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::CORTICAL_COUNT_HEADER_SIZE;

        for _cortical_index in 0..number_cortical_areas {
            let cortical_id = CorticalID::from_bytes(
                <&[u8; 6]>::try_from(&bytes[reading_header_index..reading_header_index + 6]).unwrap()
            )?;
            let data_start_reading: usize = LittleEndian::read_u32(&bytes[reading_header_index + 6..reading_header_index + 10]) as usize;
            let number_bytes_to_read: usize = LittleEndian::read_u32(&bytes[reading_header_index + 10..reading_header_index + 14]) as usize;

            if bytes.len() < data_start_reading + number_bytes_to_read {
                return Err(FeagiBytesError::UnableToSerializeBytes("Byte structure for NeuronCategoricalXYZPV1 is too short to fit the data the header says it contains!".into()).into());
            }

            let neuron_bytes = &bytes[data_start_reading..data_start_reading + number_bytes_to_read];
            let bytes_length = neuron_bytes.len();
            if bytes_length % NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON != 0 {
                return Err(FeagiBytesError::UnableToSerializeBytes("Byte structure for NeuronCategoricalXYZPV1 seems invalid! Size is nonsensical given neuron data size!".into()).into());
            }
            
            if bytes_length % NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON != 0 {
                return Err(FeagiBytesError::UnableToSerializeBytes("As NeuronXYCPArrays contains 4 internal arrays of equal length, each of elements of 4 bytes each (uint32 and float), the input byte array must be divisible by 16!".into()).into());
            }
            let x_end = bytes_length / 4; // q1
            let y_end = bytes_length / 2; // q2
            let z_end = x_end * 3; // q3

            // Create vectors using byteorder to avoid alignment issues
            let num_neurons = bytes_length / NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON;
            let mut x_coords: Vec<u32> = Vec::with_capacity(num_neurons);
            let mut y_coords: Vec<u32> = Vec::with_capacity(num_neurons);
            let mut z_coords: Vec<u32> = Vec::with_capacity(num_neurons);
            let mut potentials: Vec<f32> = Vec::with_capacity(num_neurons);
            
            for i in 0..num_neurons {
                let x_start = i * 4;
                let y_start = x_end + x_start;
                let z_start = y_end + x_start;
                let p_start = z_end + x_start;

                x_coords.push(LittleEndian::read_u32(&neuron_bytes[x_start..x_start + 4]));
                y_coords.push(LittleEndian::read_u32(&neuron_bytes[y_start..y_start + 4]));
                z_coords.push(LittleEndian::read_u32(&neuron_bytes[z_start..z_start + 4]));
                potentials.push(LittleEndian::read_f32(&neuron_bytes[p_start..p_start + 4]));
            }

            let neurons = NeuronXYZPArrays::new_from_vectors(
                x_coords,
                y_coords,
                z_coords,
                potentials,
            )?;

            output.insert(cortical_id, neurons);
            reading_header_index += Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        };
        
        Ok(output)
    }
}

impl CorticalMappedXYZPNeuronData {
    /// Binary structure type identifier for serialization.
    const BYTE_STRUCT_TYPE: FeagiByteStructureType = FeagiByteStructureType::NeuronCategoricalXYZP;
    /// Binary structure version for compatibility checking.
    const BYTE_STRUCT_VERSION: u8 = 1;
    /// Size in bytes of each cortical area header in binary format.
    const BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE: usize = 14;
    /// Size in bytes of the cortical count field in binary format.
    const CORTICAL_COUNT_HEADER_SIZE: usize = 2;
    
    /// Creates a new empty neuron data collection.
    ///
    /// This creates a new instance with an empty hash map, suitable for
    /// dynamic addition of cortical areas as needed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
    ///
    /// let neuron_data = CorticalMappedXYZPNeuronData::new();
    /// assert_eq!(neuron_data.get_number_contained_areas(), 0);
    /// ```
    pub fn new() -> CorticalMappedXYZPNeuronData {
        CorticalMappedXYZPNeuronData {mappings: HashMap::new()}
    }
    
    /// Creates a new neuron data collection with pre-allocated capacity.
    ///
    /// This is more efficient when the approximate number of cortical areas
    /// is known in advance, as it reduces hash map reallocations.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Expected number of cortical areas
    ///
    /// # Examples
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
    ///
    /// // Pre-allocate for a brain with 100 cortical areas
    /// let neuron_data = CorticalMappedXYZPNeuronData::new_with_capacity(100);
    /// assert_eq!(neuron_data.get_number_contained_areas(), 0);
    /// ```
    pub fn new_with_capacity(capacity: usize) -> CorticalMappedXYZPNeuronData {
        CorticalMappedXYZPNeuronData {mappings: HashMap::with_capacity(capacity)}
    }
    
    /// Returns the number of cortical areas currently stored.
    ///
    /// # Returns
    ///
    /// The count of cortical areas that have neuron data.
    pub fn get_number_contained_areas(&self) -> usize {
        self.mappings.len()
    }
    
    /// Inserts neuron data for a cortical area.
    ///
    /// If the cortical area already exists, its data will be replaced.
    ///
    /// # Arguments
    ///
    /// * `cortical_id` - Unique identifier for the cortical area
    /// * `neuron_data` - Collection of neurons for this cortical area
    ///
    /// # Returns
    ///
    /// `true` if the cortical area already existed (data was replaced),
    /// `false` if this is a new cortical area.
    pub fn insert(&mut self, cortical_id: CorticalID, neuron_data: NeuronXYZPArrays) -> bool {
        self.mappings.insert(cortical_id, neuron_data).is_some()
    }
    
    /// Checks if a cortical area has neuron data.
    ///
    /// # Arguments
    ///
    /// * `cortical_id` - Cortical area identifier to check
    ///
    /// # Returns
    ///
    /// `true` if the cortical area exists, `false` otherwise.
    pub fn contains(&self, cortical_id: &CorticalID) -> bool {
        self.mappings.contains_key(cortical_id)
    }
    
    /// Gets an immutable reference to neuron data for a cortical area.
    ///
    /// # Arguments
    ///
    /// * `cortical_id` - Cortical area identifier
    ///
    /// # Returns
    ///
    /// `Some(&NeuronXYZPArrays)` if the cortical area exists, `None` otherwise.
    pub fn borrow(&self, cortical_id: &CorticalID) -> Option<&NeuronXYZPArrays> {
        self.mappings.get(cortical_id)
    }
    
    /// Gets a mutable reference to neuron data for a cortical area.
    ///
    /// # Arguments
    ///
    /// * `cortical_id` - Cortical area identifier
    ///
    /// # Returns
    ///
    /// `Some(&mut NeuronXYZPArrays)` if the cortical area exists, `None` otherwise.
    pub fn borrow_mut(&mut self, cortical_id: &CorticalID) -> Option<&mut NeuronXYZPArrays> {
        self.mappings.get_mut(&cortical_id)
    }
    
    /// Gets a mutable reference to neuron data, creating or clearing as needed.
    ///
    /// This method ensures that the cortical area exists and has cleared neuron data.
    /// If the cortical area doesn't exist, it creates new neuron arrays with the
    /// specified capacity. If it exists, it clears the existing data.
    ///
    /// # Arguments
    ///
    /// * `cortical_id` - Cortical area identifier
    /// * `estimated_neuron_count` - Capacity for new neuron arrays if creation is needed
    ///
    /// # Returns
    ///
    /// A mutable reference to the neuron arrays, guaranteed to be empty and ready for use.
    pub fn ensure_clear_and_borrow_mut(&mut self, cortical_id: &CorticalID, estimated_neuron_count: usize) -> &mut NeuronXYZPArrays {
        if self.mappings.contains_key(cortical_id) {
            let neurons = self.mappings.get_mut(cortical_id).unwrap();
            neurons.reset_indexes();
            return neurons;
        }
        _ = self.mappings.insert(cortical_id.clone(), NeuronXYZPArrays::new(estimated_neuron_count).unwrap());
        self.mappings.get_mut(cortical_id).unwrap()
    }
}