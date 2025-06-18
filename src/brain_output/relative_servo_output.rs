use std::ops::RangeInclusive;
use crate::error::DataProcessingError;
use crate::neuron_data::neuron_arrays::NeuronXYZPArrays;
use crate::genome_definitions::CorticalDimensions;

/// Represents a relative servo output that processes neuron data for servo control.
/// This structure handles the conversion of neuron activations into relative servo positions
/// based on the cortical dimensions and neuron data.
#[derive(Clone)]
pub struct RelativeServoOutput {
    neuron_data: NeuronXYZPArrays,
    dimensions: CorticalDimensions
}

impl RelativeServoOutput {
    /// Creates a new RelativeServoOutput instance.
    ///
    /// # Arguments
    /// * `neuron_data` - The neuron data containing activation values
    /// * `dimensions` - The cortical dimensions that define the structure of the neural data
    ///
    /// # Returns
    /// * `Result<RelativeServoOutput, DataProcessingError>` - A new instance or an error if the dimensions are invalid
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_output::relative_servo_output::RelativeServoOutput;
    /// use feagi_core_data_structures_and_processing::genome_definitions::CorticalDimensions;
    /// use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
    ///
    /// let dimensions = CorticalDimensions::new(4, 1, 3).unwrap(); // x must be even, y must be 1
    /// let neuron_data = NeuronXYZPArrays::new_from_resolution((4, 1, 3)).unwrap();
    /// let servo_output = RelativeServoOutput::new(neuron_data, dimensions).unwrap();
    /// ```
    pub fn new(neuron_data: NeuronXYZPArrays, dimensions: CorticalDimensions) -> Result<RelativeServoOutput, DataProcessingError> {
        dimensions.verify()?;
        if dimensions.x % 2 != 0 {
            return Err(DataProcessingError::InvalidInputBounds("Relative Servo Cortical Areas need to have their X dimensions divisible by 2!".into()))
        }
        if dimensions.y != 1 {
            return Err(DataProcessingError::InvalidInputBounds("Relative Servo Cortical Areas need to have their y dimensions equal to 1!".into()))
        }
        Ok(RelativeServoOutput {
            neuron_data,
            dimensions
        })
    }
    
    /// Returns the number of servo channels available.
    ///
    /// # Returns
    /// * `usize` - The number of channels (x dimension divided by 2)
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_output::relative_servo_output::RelativeServoOutput;
    /// use feagi_core_data_structures_and_processing::genome_definitions::CorticalDimensions;
    /// use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
    ///
    /// let dimensions = CorticalDimensions::new(4, 1, 3).unwrap();
    /// let neuron_data = NeuronXYZPArrays::new_from_resolution((4, 1, 3)).unwrap();
    /// let servo_output = RelativeServoOutput::new(neuron_data, dimensions).unwrap();
    /// assert_eq!(servo_output.get_channel_count(), 2);
    /// ```
    pub fn get_channel_count(&self) -> usize {
        self.dimensions.x / 2
    }
    
    /// Calculates the float value for a specific channel based on neuron activations.
    /// The value is computed by considering neuron positions and their activation values.
    ///
    /// # Arguments
    /// * `channel` - The channel index to calculate the value for
    ///
    /// # Returns
    /// * `Result<f32, DataProcessingError>` - The calculated float value or an error if the channel is invalid
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_output::relative_servo_output::RelativeServoOutput;
    /// use feagi_core_data_structures_and_processing::genome_definitions::CorticalDimensions;
    /// use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
    /// use feagi_core_data_structures_and_processing::neuron_data::neurons::NeuronXYZP;
    ///
    /// let dimensions = CorticalDimensions::new(4, 1, 3).unwrap();
    /// let mut neuron_data = NeuronXYZPArrays::new_from_resolution((4, 1, 3)).unwrap();
    /// neuron_data.add_neuron(&NeuronXYZP::new(0, 0, 0, 0.5)); // Positive activation
    /// neuron_data.add_neuron(&NeuronXYZP::new(1, 0, 1, 0.3)); // Negative activation
    ///
    /// let servo_output = RelativeServoOutput::new(neuron_data, dimensions).unwrap();
    /// let value = servo_output.get_float_value_from_channel(0).unwrap();
    /// ```
    pub fn get_float_value_from_channel(&self, channel: usize) -> Result<f32, DataProcessingError> {
        let channel_neurons = self.filter_neurons_for_channel(channel)?;
        
        if channel_neurons.is_empty() {
            return Ok(0.0);
        }

        let number_activations: f32 = channel_neurons.get_number_of_neurons_used() as f32;
        let cortical_depth: f32 = self.dimensions.z as f32;
        
        let mut output: f32 = 0.0;
        
        for neuron in channel_neurons.iter() {
            let activation_feagi_index: f32 = (neuron.z + 1) as f32;
            let sign: f32 = if neuron.x % 2 == 0 { 1.0 } else { -1.0 }; // TODO Make this branchless
            output += neuron.p * activation_feagi_index / cortical_depth * sign;
        };
        output /= number_activations;
        
        Ok(output)
    }
    
    /// Calculates float values for all available channels.
    ///
    /// # Returns
    /// * `Result<Vec<f32>, DataProcessingError>` - A vector of float values for all channels
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_output::relative_servo_output::RelativeServoOutput;
    /// use feagi_core_data_structures_and_processing::genome_definitions::CorticalDimensions;
    /// use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
    /// use feagi_core_data_structures_and_processing::neuron_data::neurons::NeuronXYZP;
    ///
    /// let dimensions = CorticalDimensions::new(4, 1, 3).unwrap();
    /// let mut neuron_data = NeuronXYZPArrays::new_from_resolution((4, 1, 3)).unwrap();
    /// neuron_data.add_neuron(&NeuronXYZP::new(0, 0, 0, 0.5));
    /// neuron_data.add_neuron(&NeuronXYZP::new(1, 0, 1, 0.3));
    ///
    /// let servo_output = RelativeServoOutput::new(neuron_data, dimensions).unwrap();
    /// let values = servo_output.get_float_values_for_all_channels().unwrap();
    /// assert_eq!(values.len(), 2);
    /// ```
    pub fn get_float_values_for_all_channels(&self) -> Result<Vec<f32>, DataProcessingError> {
        let num_channels = self.get_channel_count();
        let mut output: Vec<f32> = Vec::with_capacity(num_channels);
        for i in 0..num_channels { // TODO this can be parallelized
            output.push(self.get_float_value_from_channel(i)?);
        };
        Ok(output)
    }
    
    /// Replaces the current neuron data with new data.
    ///
    /// # Arguments
    /// * `neuron_data` - The new neuron data to use
    ///
    /// # Returns
    /// * `Result<(), DataProcessingError>` - Success or an error if the operation fails
    ///
    /// # Examples
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_output::relative_servo_output::RelativeServoOutput;
    /// use feagi_core_data_structures_and_processing::genome_definitions::CorticalDimensions;
    /// use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
    ///
    /// let dimensions = CorticalDimensions::new(4, 1, 3).unwrap();
    /// let initial_data = NeuronXYZPArrays::new_from_resolution((4, 1, 3)).unwrap();
    /// let mut servo_output = RelativeServoOutput::new(initial_data, dimensions).unwrap();
    ///
    /// let new_data = NeuronXYZPArrays::new_from_resolution((4, 1, 3)).unwrap();
    /// servo_output.overwrite_all_neuron_data(new_data).unwrap();
    /// ```
    pub fn overwrite_all_neuron_data(&mut self, neuron_data: NeuronXYZPArrays) -> Result<(), DataProcessingError> {
        self.neuron_data = neuron_data;
        Ok(())
    }
    
    /// Filters neurons for a specific channel based on their location.
    ///
    /// # Arguments
    /// * `channel` - The channel index to filter neurons for
    ///
    /// # Returns
    /// * `Result<NeuronXYZPArrays, DataProcessingError>` - Filtered neuron data or an error if the channel is invalid
    fn filter_neurons_for_channel(&self, channel: usize) -> Result<NeuronXYZPArrays, DataProcessingError> {
        if channel > self.get_channel_count() {
            return Err(DataProcessingError::InvalidInputBounds(format!("Channel {} is greater than maximum channel count of {}!", channel, self.get_channel_count())))
        }
        let channel: u32 = channel as u32;
        let x_range: RangeInclusive<u32> = channel..=channel + 1;
        let y_range: RangeInclusive<u32> = 0..=1;
        let z_range: RangeInclusive<u32> = 0..=self.dimensions.z as u32 - 1;
        
        self.neuron_data.filter_neurons_by_location_bounds(x_range, y_range, z_range)
    }
    
    
}