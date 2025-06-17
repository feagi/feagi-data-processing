use std::ops::RangeInclusive;
use crate::error::DataProcessingError;
use crate::neuron_data::neuron_arrays::NeuronXYZPArrays;
use crate::cortical_data::CorticalDimensions;

#[derive(Clone)]
pub struct RelativeServoOutput {
    neuron_data: NeuronXYZPArrays,
    dimensions: CorticalDimensions
}

impl RelativeServoOutput {
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
    
    pub fn get_channel_count(&self) -> usize {
        self.dimensions.x / 2
    }
    
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
    
    pub fn get_float_values_for_all_channels(&self) -> Result<Vec<f32>, DataProcessingError> {
        let num_channels = self.get_channel_count();
        let mut output: Vec<f32> = Vec::with_capacity(num_channels);
        for i in 0..num_channels { // TODO this can be parallelized
            output.push(self.get_float_value_from_channel(i)?);
        };
        Ok(output)
    }
    
    pub fn overwrite_all_neuron_data(&mut self, neuron_data: NeuronXYZPArrays) -> Result<(), DataProcessingError> {
        self.neuron_data = neuron_data;
        Ok(())
    }
    
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