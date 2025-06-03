use std::collections::HashMap;
use byteorder::{ByteOrder, LittleEndian};
use crate::error::DataProcessingError;
use crate::cortical_data::CorticalID;
use crate::byte_structures::{FeagiByteStructureType, GLOBAL_HEADER_SIZE};
use crate::byte_structures::feagi_byte_structure::FeagiByteStructureCompatible;
use super::neuron_arrays::{NeuronXYZPArrays};

pub struct CorticalMappedXYZPNeuronData {
    pub mappings: HashMap<CorticalID, NeuronXYZPArrays>,
}

impl FeagiByteStructureCompatible for CorticalMappedXYZPNeuronData {
    fn get_type(&self) -> FeagiByteStructureType { FeagiByteStructureType::NeuronCategoricalXYZP }
    fn get_version(&self) -> u8 { 1 }
    fn overwrite_feagi_byte_structure_slice(&self, slice: &mut [u8]) -> Result<usize, DataProcessingError> {
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;
        const PER_CORTICAL_HEADER_DESCRIPTOR_SIZE: usize = 8;
        
        let num_bytes_needed: usize = self.max_number_bytes_needed();
        if slice.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store neuron XYZP data! Need {} bytes but given {}!", num_bytes_needed, slice.len())));
        }

        slice[0] = self.get_type() as u8;
        slice[1] = self.get_version();

        let number_cortical_areas: usize = self.mappings.len();
        LittleEndian::write_u16(&mut slice[2..4], number_cortical_areas as u16);

        let mut subheader_write_index: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE;
        let mut neuron_data_write_index: u32 = subheader_write_index as u32 + (number_cortical_areas as u32 * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

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
            subheader_write_index += PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        };
        let wasted_space = slice.len() - num_bytes_needed;
        Ok(wasted_space)
    }

    fn max_number_bytes_needed(&self) -> usize {
        let mut number_neurons: usize = 0;
        for neuron_set in self.mappings.values() {
            number_neurons += neuron_set.get_number_of_neurons_used();
        }
        number_neurons * NeuronXYZPArrays::NUMBER_BYTES_PER_NEURON
    }
}

impl CorticalMappedXYZPNeuronData {
    pub fn new() -> CorticalMappedXYZPNeuronData {
        CorticalMappedXYZPNeuronData {mappings: HashMap::new()}
    }
    
    pub fn insert(&mut self, cortical_id: CorticalID, neuron_data: NeuronXYZPArrays) -> Option<NeuronXYZPArrays> {
        self.mappings.insert(cortical_id, neuron_data)
    }
    
    pub fn get_mut(&mut self, cortical_id: &CorticalID) -> Option<&mut NeuronXYZPArrays> {
        self.mappings.get_mut(&cortical_id)
    }
}