use std::collections::HashMap;
use byteorder::{ByteOrder, LittleEndian};
use crate::error::DataProcessingError;
use crate::cortical_data::CorticalID;
use crate::byte_structures::{FeagiByteStructureCompatible, FeagiByteStructureType, GLOBAL_HEADER_SIZE};
use crate::byte_structures::feagi_byte_structure::{verify_matching_structure_type_and_version, FeagiByteStructure};
use super::neuron_arrays::NeuronXYZPArrays;

#[derive(Clone)]
pub struct CorticalMappedXYZPNeuronData {
    pub mappings: HashMap<CorticalID, NeuronXYZPArrays>,
}

impl FeagiByteStructureCompatible for CorticalMappedXYZPNeuronData {
    fn get_type(&self) -> FeagiByteStructureType { Self::BYTE_STRUCT_TYPE }
    fn get_version(&self) -> u8 { Self::BYTE_STRUCT_VERSION }
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, DataProcessingError> {
        
        if self.mappings.len() == 0 {
            return Err(DataProcessingError::InvalidByteStructure("Cannot generate a byte structure export with an empty cortical mappings object!".into()))
        }
        
        let num_bytes_needed: usize = self.max_number_bytes_needed();
        if slice.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store neuron XYZP data! Need {} bytes but given {}!", num_bytes_needed, slice.len())));
        }

        slice[0] = self.get_type() as u8;
        slice[1] = self.get_version();

        let number_cortical_areas: usize = self.mappings.len();
        LittleEndian::write_u16(&mut slice[2..4], number_cortical_areas as u16);

        let mut subheader_write_index: usize = GLOBAL_HEADER_SIZE + Self::CORTICAL_COUNT_HEADER_SIZE;
        let mut neuron_data_write_index: u32 = subheader_write_index as u32 + (number_cortical_areas as u32 * Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        for (cortical_id, neuron_data) in &self.mappings {
            // Write cortical subheader
            cortical_id.write_bytes_at(&mut slice[subheader_write_index .. subheader_write_index + 6])?;
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
        GLOBAL_HEADER_SIZE + Self::CORTICAL_COUNT_HEADER_SIZE + 
            (self.get_number_contained_areas() * Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as usize) +
            bytes_needed_for_neurons
    }

    fn new_from_feagi_byte_structure(feagi_byte_structure: &FeagiByteStructure) -> Result<Self, DataProcessingError> {
        verify_matching_structure_type_and_version(&feagi_byte_structure,
                                                   Self::BYTE_STRUCT_TYPE,
                                                   Self::BYTE_STRUCT_VERSION)?;
        
        let bytes = feagi_byte_structure.borrow_data_as_slice();
        let number_cortical_areas: u16 = LittleEndian::read_u16(&bytes[2..4]);

        let min_array_length_with_cortical_headers: usize = GLOBAL_HEADER_SIZE +  Self::CORTICAL_COUNT_HEADER_SIZE +
            (Self::BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE * number_cortical_areas as usize);

        if bytes.len() < min_array_length_with_cortical_headers {
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte structure for NeuronCategoricalXYZPV1 needs a length of {} to fit just the cortical details header, but is a length of {}",
                                                                         min_array_length_with_cortical_headers, bytes.len())));
        }

        let number_cortical_areas: usize = number_cortical_areas as usize;
        let mut output: CorticalMappedXYZPNeuronData = CorticalMappedXYZPNeuronData::new_with_capacity(number_cortical_areas);
        
        let mut reading_header_index: usize = GLOBAL_HEADER_SIZE + Self::CORTICAL_COUNT_HEADER_SIZE;

        for _cortical_index in 0..number_cortical_areas {
            let cortical_id = CorticalID::from_bytes_at(
                &bytes[reading_header_index..reading_header_index + 6]
            )?;

            let data_start_reading: usize = LittleEndian::read_u32(&bytes[reading_header_index + 6..reading_header_index + 10]) as usize;
            let number_bytes_to_read: usize = LittleEndian::read_u32(&bytes[reading_header_index + 10..reading_header_index + 14]) as usize;

            if bytes.len() < data_start_reading + number_bytes_to_read {
                return Err(DataProcessingError::InvalidByteStructure("Byte structure for NeuronCategoricalXYZPV1 is too short to fit the data the header says it contains!".into()));
            }

            let neuron_bytes = &bytes[data_start_reading..data_start_reading + number_bytes_to_read];
            let bytes_length = neuron_bytes.len();
            if bytes_length % NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON != 0 {
                return Err(DataProcessingError::InvalidByteStructure("Byte structure for NeuronCategoricalXYZPV1 seems invalid! Size is nonsensical given neuron data size!".into()));
            }
            
            if bytes_length % NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON != 0 {
                return Err(DataProcessingError::InvalidByteStructure("As NeuronXYCPArrays contains 4 internal arrays of equal length, each of elements of 4 bytes each (uint32 and float), the input byte array must be divisible by 16!".into()));
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
    const BYTE_STRUCT_TYPE: FeagiByteStructureType = FeagiByteStructureType::NeuronCategoricalXYZP;
    const BYTE_STRUCT_VERSION: u8 = 1;
    const BYTE_PER_CORTICAL_HEADER_DESCRIPTOR_SIZE: usize = 14;
    const CORTICAL_COUNT_HEADER_SIZE: usize = 2;
    
    
    pub fn new() -> CorticalMappedXYZPNeuronData {
        CorticalMappedXYZPNeuronData {mappings: HashMap::new()}
    }
    
    pub fn new_with_capacity(capacity: usize) -> CorticalMappedXYZPNeuronData {
        CorticalMappedXYZPNeuronData {mappings: HashMap::with_capacity(capacity)}
    }
    
    pub fn get_number_contained_areas(&self) -> usize {
        self.mappings.len()
    }
    
    pub fn insert(&mut self, cortical_id: CorticalID, neuron_data: NeuronXYZPArrays) -> Option<NeuronXYZPArrays> {
        self.mappings.insert(cortical_id, neuron_data)
    }
    
    pub fn contains(&self, cortical_id: CorticalID) -> bool {
        self.mappings.contains_key(&cortical_id)
    }
    
    pub fn borrow(&self, cortical_id: &CorticalID) -> Option<&NeuronXYZPArrays> {
        self.mappings.get(cortical_id)
    }
    
    pub fn borrow_mut(&mut self, cortical_id: &CorticalID) -> Option<&mut NeuronXYZPArrays> {
        self.mappings.get_mut(&cortical_id)
    }
}