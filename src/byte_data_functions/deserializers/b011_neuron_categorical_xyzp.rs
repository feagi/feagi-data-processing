use byteorder::{ByteOrder, LittleEndian};
use crate::error::DataProcessingError;
use crate::neuron_data::{CorticalMappedNeuronData, NeuronXYCPArrays};
use crate::cortical_data::CorticalID;
use super::{FeagiByteDeserializer, FeagiByteStructureType, verify_header_of_full_structure_bytes};

pub struct NeuronCategoricalXYZPDeserializerV1<'internal_bytes> {
    data_slice: &'internal_bytes [u8],
}

impl FeagiByteDeserializer for NeuronCategoricalXYZPDeserializerV1<'_> {
    fn get_id(&self) -> u8 {FeagiByteStructureType::NeuronCategoricalXYZP as u8}
    fn get_version(&self) -> u8 {1}
}

impl<'internal_bytes> NeuronCategoricalXYZPDeserializerV1<'internal_bytes> {
    pub fn from_data_slice(data_slice: & 'internal_bytes[u8]) -> Result<NeuronCategoricalXYZPDeserializerV1<'internal_bytes>, DataProcessingError> {
        verify_header_of_full_structure_bytes(data_slice, FeagiByteStructureType::NeuronCategoricalXYZP, 1)?;
        Ok(NeuronCategoricalXYZPDeserializerV1 { data_slice })
    }

    fn to_cortical_mapped_neuron_data(&self) -> Result<CorticalMappedNeuronData, DataProcessingError> {
        // We don't have to verify the global header since that was checked on this struct being created
        // We also know it has at least 4 bytes
        const GLOBAL_HEADER_SIZE: usize = crate::byte_data_functions::GLOBAL_HEADER_SIZE;
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;

        let number_cortical_areas: u16 = LittleEndian::read_u16(&self.data_slice[2..4]);

        let min_array_length_with_cortical_headers: usize = GLOBAL_HEADER_SIZE +  CORTICAL_COUNT_HEADER_SIZE +
            (NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE * number_cortical_areas as usize);

        if self.data_slice.len() < min_array_length_with_cortical_headers {
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte structure for NeuronCategoricalXYZPV1 needs a length of {} to fit just the cortical details header, but is a length of {}",
            min_array_length_with_cortical_headers, self.data_slice.len())));
        }

        let number_cortical_areas: usize = number_cortical_areas as usize;

        let mut output: CorticalMappedNeuronData = CorticalMappedNeuronData::with_capacity(number_cortical_areas);

        let mut reading_index: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE;
        for cortical_index in 0..number_cortical_areas {
            let cortical_id = CorticalID::from_bytes_at(
                &self.data_slice[reading_index..reading_index + 6]
            )?;
            let data_start_reading: usize = LittleEndian::read_u32(&self.data_slice[reading_index + 6..reading_index + 10]) as usize;
            let number_bytes_to_read: usize = LittleEndian::read_u32(&self.data_slice[reading_index + 10..reading_index + 14]) as usize * NeuronXYCPArrays::NUMBER_BYTES_PER_NEURON;
            
            if self.data_slice.len() < min_array_length_with_cortical_headers + data_start_reading + number_bytes_to_read {
                return Err(DataProcessingError::InvalidByteStructure("Byte structure for NeuronCategoricalXYZPV1 is too short to fit the data the header says it contains!".into()));
            }

            let neurons = NeuronXYCPArrays::new_from_bytes(
                &self.data_slice[data_start_reading..data_start_reading + number_bytes_to_read]
            )?;

            output.insert(cortical_id, neurons);
            reading_index += NeuronXYCPArrays::PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        };

        Ok(output)

    }

}