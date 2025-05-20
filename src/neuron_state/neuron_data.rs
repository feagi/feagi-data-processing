use std::collections::HashMap;
use crate::cortical_area_state::cortical_data::CorticalID;
use crate::Error::DataProcessingError;

pub type CorticalMappedNeuronData = HashMap<CorticalID, NeuronYXCPArrays>;


pub type XYCPOrderedVectorWriteTargets<'a> = (&'a mut Vec<u32>, &'a mut Vec<u32>, &'a mut Vec<u32>, &'a mut Vec<f32>);

#[derive(Clone)]
pub struct NeuronYXCPArrays{
    x: Vec<u32>, // Remember, FEAGI is cartesian!
    y: Vec<u32>,
    c: Vec<u32>,
    p: Vec<f32>,
    max_number_neurons: usize,
}

impl NeuronYXCPArrays{
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> Result<Self, DataProcessingError> {
        const NUMBER_BYTES_PER_NEURON: usize = 16;
        if maximum_number_of_neurons_possibly_needed == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Given number of neurons possible must be greater than 0!".into()));
        };
        Ok(NeuronYXCPArrays{
            y: Vec::with_capacity(NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
            x: Vec::with_capacity(NUMBER_BYTES_PER_NEURON * maximum_number_of_neurons_possibly_needed),
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
        let mut x_data_write_index: usize = header_write_index + (number_cortical_areas * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE);
        
        
        let mut data_write_index: u32 = 4 + (number_cortical_areas as u32 * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);
        
        // fill in cortical descriptors header
        for (cortical_id, neuron_data) in &mapped_data {
            // Calculate locations
            let reading_start: u32 = data_write_index;
            let reading_length: u32 = neuron_data.get_number_of_neurons_used() as u32 * PER_NEURON_XYZP_SIZE as u32;
            let reading_start_bytes: [u8; 4] = reading_start.to_le_bytes();
            let reading_length_bytes: [u8; 4] = reading_length.to_le_bytes();

            // Write cortical subheader
            cortical_id.write_bytes_at(&mut output[header_write_index..header_write_index + 6]);
            output[header_write_index + 6.. header_write_index + 10].copy_from_slice(&reading_start_bytes);
            output[header_write_index + 10.. header_write_index + 14].copy_from_slice(&reading_length_bytes);
            
            // Write data
            neuron_data.write_data_to_bytes(&mut output[reading_start as usize .. (reading_start + reading_length) as usize])?;
            
            // update indexes
            data_write_index += reading_length;
        }
        
        Ok(output)
    }
    
    pub fn new_from_resolution(resolution: (usize, usize, usize)) -> Result<Self, DataProcessingError> {
        return crate::neuron_state::neuron_data::NeuronYXCPArrays::new(resolution.0 * resolution.1 * resolution.2);
    }
    
    pub fn get_as_xycp_vectors(&mut self) -> XYCPOrderedVectorWriteTargets {
        (&mut self.x, &mut self.y, &mut self.c, &mut self.p) // This isnt the best design, someone could write vectors of different sizes
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
            return return Err(DataProcessingError::InternalError("Internal YXCP Arrays do not have equal lengths!".into()));
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
            return Err(DataProcessingError::InternalError("Invalid number of bytes passed to write neuronal YXCP data to!".into()))
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















/*


pub type CorticalMappedNeuronPotentialCollectionXYZ = HashMap<CorticalID, NeuronPotentialCollectionXYZ>;

/// Represents a single neuron with its 3D coordinates and potential value
///
/// This structure stores the position and potential (voltage) of an individual neuron
#[derive(Debug)]
pub struct NeuronPotentialXYZ {
    /// X-coordinate of the neuron
    coordinate_x: u32,
    /// Y-coordinate of the neuron
    coordinate_y: u32,
    /// Z-coordinate of the neuron
    coordinate_z: u32,
    /// Potential (voltage) value of the neuron
    potential: f32
}

impl NeuronPotentialXYZ {
    /// Creates a new NeuronPotentialXYZ instance
    ///
    /// # Arguments
    ///
    /// * `coordinate_x` - X-coordinate of the neuron
    /// * `coordinate_y` - Y-coordinate of the neuron
    /// * `coordinate_z` - Z-coordinate of the neuron
    /// * `potential` - Potential (voltage) value of the neuron
    ///
    /// # Returns
    ///
    /// * `Result<NeuronPotentialXYZ, &'static str>` - The created instance or an error message
    pub fn new(coordinate_x: u32, coordinate_y: u32, coordinate_z: u32, potential: f32) -> Result<NeuronPotentialXYZ, &'static str> {
        Ok(NeuronPotentialXYZ{
            coordinate_x,
            coordinate_y,
            coordinate_z,
            potential
        })
    }
    
    /// Converts a vector of individual NeuronPotentialXYZ instances into a collection
    ///
    /// This function takes a vector of individual neurons and converts it into
    /// a more efficient parallel-array representation.
    ///
    /// # Arguments
    ///
    /// * `vector` - Vector of NeuronPotentialXYZ instances to convert
    ///
    /// # Returns
    ///
    /// * `Result<NeuronPotentialCollectionXYZ, &'static str>` - The created collection or an error message
    pub fn convert_to_neuron_potential_collection_xyz(vector: Vec<NeuronPotentialXYZ>) -> Result<NeuronPotentialCollectionXYZ, &'static str> {
        let length = vector.len();
        
        let x = Array1::from_shape_fn(length, |i| {vector[i].coordinate_x});
        let y = Array1::from_shape_fn(length, |i| {vector[i].coordinate_y});
        let z = Array1::from_shape_fn(length, |i| {vector[i].coordinate_z});
        let p = Array1::from_shape_fn(length, |i| {vector[i].potential});
        
        let obj: Result<NeuronPotentialCollectionXYZ, &str> = NeuronPotentialCollectionXYZ::new(x, y, z, p);
        match obj {
            Ok(_) => {Ok(obj.unwrap())}
            Err(_) => {Err(obj.unwrap_err())}
        }
        
    }
}



/// Represents a set of neurons with their 3D coordinates and (voltage) potential values
///
/// This structure holds parallel arrays of x, y, z coordinates and neuron voltage potentials,
/// where the values at the same index across arrays represent a single neuron.
#[derive(Debug)]
pub struct NeuronPotentialCollectionXYZ {
    /// X-coordinates of the neurons, ordered
    coordinates_x: Array1<u32>,
    /// Y-coordinates of the neurons, ordered
    coordinates_y: Array1<u32>,
    /// Z-coordinates of the neurons, ordered
    coordinates_z: Array1<u32>,
    /// Potential values of the neurons, ordered
    potentials: Array1<f32>,
}

impl NeuronPotentialCollectionXYZ {
    /// Creates a new NeuronSetXYZ instance
    ///
    /// # Arguments
    ///
    /// * `x` - Array of X-coordinates for each neuron
    /// * `y` - Array of Y-coordinates for each neuron
    /// * `z` - Array of Z-coordinates for each neuron
    /// * `neuron_potentials` - Array of potential values for each neuron. Values possible include negatives
    ///
    /// # Returns
    ///
    /// * `Result<NeuronSetXYZ, &'static str>` - A Result containing either the constructed NeuronSetXYZ
    ///   or an error message if the input is invalid
    ///
    /// # Errors
    ///
    /// Returns an error if array length is 0
    /// Returns an error if the arrays have different lengths
    pub fn new(x: Array1<u32>, y: Array1<u32>, z: Array1<u32>, neuron_potentials: Array1<f32>) -> Result<NeuronPotentialCollectionXYZ, &'static str> {
        if x.len() == 0 {
            return Err("Arrays cannot be empty!");
        }
        
        if x.len() != neuron_potentials.len() || y.len() != neuron_potentials.len() || z.len() != neuron_potentials.len() {
            return Err("Input Arrays must have the same length!");
        }
        
        Ok(NeuronPotentialCollectionXYZ {
            coordinates_x: x,
            coordinates_y: y,
            coordinates_z: z,
            potentials: neuron_potentials,
        })
    }
    
    /// Returns the number of neurons in the collection
    ///
    /// # Returns
    ///
    /// * `usize` - The number of neurons represented in this collection
    pub fn get_number_neurons(&self) -> usize {
        self.coordinates_x.len()
    }
    
    /// Converts the parallel-array representation back to a vector of individual neurons
    ///
    /// This is the inverse operation of convert_to_neuron_potential_collection_xyz.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<NeuronPotentialXYZ>, &'static str>` - Vector of individual neurons or an error message
    pub fn convert_to_vector_of_neuron_potential_xyz(&self) ->  Result<Vec<NeuronPotentialXYZ>, &'static str> {

        Ok((0..self.coordinates_x.len())
            .map(|i| NeuronPotentialXYZ::new(
                self.coordinates_x[i],
                self.coordinates_y[i],
                self.coordinates_z[i],
                self.potentials[i],
            ).unwrap())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr1;

    // Helper function for float comparison with epsilon
    fn float_eq(a: f32, b: f32) -> bool {
        let epsilon = 1e-6;
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_neuron_potential_xyz_new() {
        let neuron = NeuronPotentialXYZ::new(1, 2, 3, 0.5).unwrap();
        assert_eq!(neuron.coordinate_x, 1);
        assert_eq!(neuron.coordinate_y, 2);
        assert_eq!(neuron.coordinate_z, 3);
        assert!(float_eq(neuron.potential, 0.5));
    }

    #[test]
    fn test_neuron_potential_collection_xyz_new() {
        let x = arr1(&[1, 2, 3]);
        let y = arr1(&[4, 5, 6]);
        let z = arr1(&[7, 8, 9]);
        let p = arr1(&[0.1, 0.2, 0.3]);

        let collection = NeuronPotentialCollectionXYZ::new(x, y, z, p).unwrap();
        assert_eq!(collection.get_number_neurons(), 3);
    }

    #[test]
    fn test_neuron_potential_collection_xyz_new_empty_arrays() {
        let x = Array1::<u32>::zeros(0);
        let y = Array1::<u32>::zeros(0);
        let z = Array1::<u32>::zeros(0);
        let p = Array1::<f32>::zeros(0);

        let result = NeuronPotentialCollectionXYZ::new(x, y, z, p);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Arrays cannot be empty!");
    }

    #[test]
    fn test_neuron_potential_collection_xyz_new_different_lengths() {
        let x = arr1(&[1, 2, 3]);
        let y = arr1(&[4, 5]);
        let z = arr1(&[7, 8, 9]);
        let p = arr1(&[0.1, 0.2, 0.3]);

        let result = NeuronPotentialCollectionXYZ::new(x, y, z, p);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input Arrays must have the same length!");
    }

    #[test]
    fn test_convert_to_collection_and_back() {
        // Create individual neurons
        let neuron1 = NeuronPotentialXYZ::new(1, 2, 3, 0.1).unwrap();
        let neuron2 = NeuronPotentialXYZ::new(4, 5, 6, 0.2).unwrap();
        let neuron3 = NeuronPotentialXYZ::new(7, 8, 9, 0.3).unwrap();
        
        let neurons = vec![neuron1, neuron2, neuron3];
        
        // Convert to collection
        let collection = NeuronPotentialXYZ::convert_to_neuron_potential_collection_xyz(neurons).unwrap();
        assert_eq!(collection.get_number_neurons(), 3);
        
        // Convert back to vector
        let neurons_converted = collection.convert_to_vector_of_neuron_potential_xyz().unwrap();
        assert_eq!(neurons_converted.len(), 3);
        
        // Verify the values are preserved
        assert_eq!(neurons_converted[0].coordinate_x, 1);
        assert_eq!(neurons_converted[0].coordinate_y, 2);
        assert_eq!(neurons_converted[0].coordinate_z, 3);
        assert!(float_eq(neurons_converted[0].potential, 0.1));
        
        assert_eq!(neurons_converted[1].coordinate_x, 4);
        assert_eq!(neurons_converted[1].coordinate_y, 5);
        assert_eq!(neurons_converted[1].coordinate_z, 6);
        assert!(float_eq(neurons_converted[1].potential, 0.2));
        
        assert_eq!(neurons_converted[2].coordinate_x, 7);
        assert_eq!(neurons_converted[2].coordinate_y, 8);
        assert_eq!(neurons_converted[2].coordinate_z, 9);
        assert!(float_eq(neurons_converted[2].potential, 0.3));
    }
}

 */