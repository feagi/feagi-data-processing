use std::collections::HashMap;
use crate::cortical_area_state::cortical_data::CorticalID;
use crate::Error::DataProcessingError;

pub type CorticalMappedNeuronData = HashMap<CorticalID, NeuronXYCPArrays>;


pub type XYCPOrderedVectorWriteTargets<'a> = (&'a mut Vec<u32>, &'a mut Vec<u32>, &'a mut Vec<u32>, &'a mut Vec<f32>);

#[derive(Clone)]
pub struct NeuronXYCPArrays{
    x: Vec<u32>, // Remember, FEAGI is cartesian!
    y: Vec<u32>,
    c: Vec<u32>,
    p: Vec<f32>,
    max_number_neurons: usize,
}

impl NeuronXYCPArrays{
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> Result<Self, DataProcessingError> {
        const NUMBER_BYTES_PER_NEURON: usize = 16;
        if maximum_number_of_neurons_possibly_needed == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Given number of neurons possible must be greater than 0!".into()));
        };
        Ok(NeuronXYCPArrays{
            x: Vec::with_capacity(NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            y: Vec::with_capacity(NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            c: Vec::with_capacity(NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            p: Vec::with_capacity(NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            max_number_neurons: maximum_number_of_neurons_possibly_needed
        })
    }

    pub fn cortical_mapped_neuron_data_to_bytes(mapped_data: CorticalMappedNeuronData) -> Result<Vec<u8>, DataProcessingError> {
        const BYTE_STRUCT_ID: u8 = 11;
        const BYTE_STRUCT_VERSION: u8 = 1;
        const GLOBAL_HEADER_SIZE: usize = 2;
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;
        const PER_CORTICAL_HEADER_DESCRIPTOR_SIZE: usize = 14;
        const PER_NEURON_XYZP_SIZE: usize = 16;


        // Calculate prerequisite info
        let number_cortical_areas: usize = mapped_data.len();
        let mut number_of_neurons_total: usize = 0;
        for (_, neuron_data) in &mapped_data {
            number_of_neurons_total += neuron_data.get_number_of_neurons_used();
        };

        let total_length_of_byte_structure = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE +
            (number_cortical_areas * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE) +
            (number_of_neurons_total * PER_NEURON_XYZP_SIZE);

        let mut output: Vec<u8> = vec![0; total_length_of_byte_structure];

        // Fill in constant size header
        output[0] = BYTE_STRUCT_ID;
        output[1] = BYTE_STRUCT_VERSION;

        let count_bytes: [u8; 2] = (number_cortical_areas as u16).to_le_bytes();
        output[2..4].copy_from_slice(&count_bytes);

        let mut header_write_index: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE;
        let mut data_write_index: u32 = header_write_index as u32 + (number_cortical_areas as u32 * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);

        // fill in cortical descriptors header
        for (cortical_id, neuron_data) in &mapped_data {
            // Calculate locations
            let reading_start: u32 = data_write_index;
            let reading_length: u32 = neuron_data.get_number_of_neurons_used() as u32 * PER_NEURON_XYZP_SIZE as u32;
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
            header_write_index += PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
        }

        Ok(output)
    }

    pub fn new_from_resolution(resolution: (usize, usize, usize)) -> Result<Self, DataProcessingError> {
        NeuronXYCPArrays::new(resolution.0 * resolution.1 * resolution.2)
    }

    pub fn get_as_xycp_vectors(&mut self) -> XYCPOrderedVectorWriteTargets {
        (&mut self.x, &mut self.y, &mut self.c, &mut self.p) // TODO This isn't the best design, someone could write vectors of different sizes
    }

    pub fn get_max_possible_number_of_neurons_out(&self) -> usize {
        self.max_number_neurons
    }

    pub fn reset_indexes(&mut self) {
        self.x.truncate(0);
        self.y.truncate(0);
        self.c.truncate(0);
        self.p.truncate(0);
    }

    pub fn validate_equal_vector_lengths(&self) -> Result<(), DataProcessingError> {
        if !((self.y.len() == self.x.len()) && (self.x.len() == self.c.len()) && (self.c.len() == self.p.len())) {
             return Err(DataProcessingError::InternalError("Internal XYCP Arrays do not have equal lengths!".into()));
        }
        Ok(())
    }

    pub fn get_number_of_neurons_used(&self) -> usize {
        self.p.len() // all of these are of qual length
    }

    fn write_data_to_bytes(&self, bytes_to_write_to: &mut [u8]) -> Result<(), DataProcessingError> {
        self.validate_equal_vector_lengths()?;
        const PER_NEURON_XYZP_SIZE: usize = 16;
        const U32_F32_LENGTH: usize = 4;
        let number_of_neurons_to_write: usize = self.get_number_of_neurons_used();
        let number_bytes_needed = PER_NEURON_XYZP_SIZE * number_of_neurons_to_write;
        if bytes_to_write_to.len() != number_bytes_needed {
            return Err(DataProcessingError::InternalError("Invalid number of bytes passed to write neuronal XYCP data to!".into()))
        }

        let mut x_offset: usize = 0;
        let mut y_offset = number_of_neurons_to_write * PER_NEURON_XYZP_SIZE / 4; // we want to be a quarter way
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