use crate::byte_structures::GLOBAL_HEADER_SIZE;
use crate::byte_structures::serializer::b001_json::JsonSerializerV1;
use crate::error::DataProcessingError;
use crate::neuron_data::{CorticalMappedNeuronData, NeuronXYCPArrays};
use super::FeagiByteSerializer;

pub struct NeuronCategoricalXYZPSerializerV1 {
    cortical_mapped_neuron_data: CorticalMappedNeuronData,
}

impl FeagiByteSerializer for NeuronCategoricalXYZPSerializerV1 {
    fn get_id(&self) -> u8 { 11 }
    fn get_version(&self) -> u8 { 1 }
    fn get_max_possible_size_when_serialized(&self) -> usize {
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

        let mut size = GLOBAL_HEADER_SIZE + NeuronCategoricalXYZPSerializerV1::CORTICAL_COUNT_HEADER_SIZE;
        for (_cortical_id, mapped_neuron_data) in self.cortical_mapped_neuron_data.iter(){
            size += NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE + mapped_neuron_data.get_number_of_bytes_needed_if_serialized()
        };
        size
    }
    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError> {
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

        let mut output: Vec<u8> = Vec::with_capacity(self.get_max_possible_size_when_serialized());
        output[0] = self.get_id();
        output[1] = self.get_version();

        let number_cortical_areas: usize = self.cortical_mapped_neuron_data.len();
        let count_bytes: [u8; 2] = (number_cortical_areas as u16).to_le_bytes();
        output[2..4].copy_from_slice(&count_bytes);

        let mut header_write_index: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE;
        let mut data_write_index: u32 = header_write_index as u32 + (number_cortical_areas as u32 * NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        // fill in cortical descriptors header
        for (cortical_id, neuron_data) in &self.cortical_mapped_neuron_data {
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
    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError> {

        let num_bytes_needed: usize = self.get_max_possible_size_when_serialized();
        if bytes_to_overwrite.len() < num_bytes_needed {
            return Err(DataProcessingError::IncompatibleInplace(format!("Not enough space given to store neuron XYZP data! Need {} bytes but given {}!", num_bytes_needed, bytes_to_overwrite.len())));
        }

        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

        bytes_to_overwrite[0] = self.get_id();
        bytes_to_overwrite[1] = self.get_version();

        let number_cortical_areas: usize = self.cortical_mapped_neuron_data.len();
        let count_bytes: [u8; 2] = (number_cortical_areas as u16).to_le_bytes();
        bytes_to_overwrite[2..4].copy_from_slice(&count_bytes);

        let mut header_write_index: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE;
        let mut data_write_index: u32 = header_write_index as u32 + (number_cortical_areas as u32 * NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        // fill in cortical descriptors header
        for (cortical_id, neuron_data) in &self.cortical_mapped_neuron_data {
            // Calculate locations
            let reading_start: u32 = data_write_index;
            let reading_length: u32 = neuron_data.get_number_of_neurons_used() as u32 * NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON as u32;
            let reading_start_bytes: [u8; 4] = reading_start.to_le_bytes();
            let reading_length_bytes: [u8; 4] = reading_length.to_le_bytes();

            // Write cortical subheader
            cortical_id.write_bytes_at(&mut bytes_to_overwrite[header_write_index..header_write_index + 6])?;
            bytes_to_overwrite[header_write_index + 6.. header_write_index + 10].copy_from_slice(&reading_start_bytes);
            bytes_to_overwrite[header_write_index + 10.. header_write_index + 14].copy_from_slice(&reading_length_bytes);

            // Write data
            neuron_data.write_data_to_bytes(&mut bytes_to_overwrite[reading_start as usize .. (reading_start + reading_length) as usize])?;

            // update indexes
            data_write_index += reading_length;
            header_write_index += NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        };

        Ok(bytes_to_overwrite.len() - num_bytes_needed)
    }

}

impl NeuronCategoricalXYZPSerializerV1 {
    pub const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

    pub fn from_cortical_mapped_neuron_data(cortical_mapped_neuron_data: CorticalMappedNeuronData) -> Result<NeuronCategoricalXYZPSerializerV1, DataProcessingError> {
        Ok(NeuronCategoricalXYZPSerializerV1{cortical_mapped_neuron_data})
    }

    pub fn as_mut(&mut self) -> &mut CorticalMappedNeuronData {
        &mut self.cortical_mapped_neuron_data
    }
}