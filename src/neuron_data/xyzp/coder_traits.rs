use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::CorticalIOChannelIndex;
use crate::io_data::{IOTypeData, IOTypeVariant};
use super::{NeuronXYZPArrays};

// Coders can be enums since they do not store values, they merely are organizational units directing
// to specific methods for reading and writing neural data

// TODO right now the multi channel functions call the single functions in a loop, while this technically works, we can do better. Right now each single channel fn iterates over all neurons for some types (floats), looking just for the specific channel it is set to. We can vectorize this

pub trait NeuronXYZPEncoder {

    fn get_data_type(&self) -> IOTypeVariant;
    
    fn write_neuron_data_single_channel(&self, wrapped_value: IOTypeData, target_to_overwrite: &mut NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError>;

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, IOTypeData>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, target_to_overwrite, channel)?;
        };
        Ok(())
    }
}

pub trait NeuronXYZPDecoder {

    fn get_data_type(&self) -> IOTypeVariant;
    
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<IOTypeData, FeagiDataProcessingError>;

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<CorticalIOChannelIndex>) -> Result<Vec<IOTypeData>, FeagiDataProcessingError> {
        let mut output: Vec<IOTypeData> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(neuron_data, channel)?);
        };
        Ok(output)
    }
}