use ndarray::{Array1};

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
    pub fn convert_to_NeuronPotentialCollectionXYZ(vector: Vec<NeuronPotentialXYZ>) -> Result<NeuronPotentialCollectionXYZ, &'static str> {
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
    /// This is the inverse operation of convert_to_NeuronPotentialCollectionXYZ.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<NeuronPotentialXYZ>, &'static str>` - Vector of individual neurons or an error message
    pub fn convert_to_vector_of_NeuronPotentialXYZ(&self) ->  Result<Vec<NeuronPotentialXYZ>, &'static str> {

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
        let collection = NeuronPotentialXYZ::convert_to_NeuronPotentialCollectionXYZ(neurons).unwrap();
        assert_eq!(collection.get_number_neurons(), 3);
        
        // Convert back to vector
        let neurons_converted = collection.convert_to_vector_of_NeuronPotentialXYZ().unwrap();
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