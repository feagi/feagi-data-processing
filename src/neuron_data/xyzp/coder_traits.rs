use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::io_cache::ChannelIndex;
use super::{NeuronXYZPArrays};

pub trait NeuronXYZPEncoder<T> {
    fn write_neuron_data_single_channel(&self, value: T, target_to_overwrite: &mut NeuronXYZPArrays, channel: ChannelIndex) -> Result<(), FeagiDataProcessingError>;

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<ChannelIndex, T>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, target_to_overwrite, channel)?;
        };
        Ok(())
    }
}

pub trait NeuronXYZPDecoder<T> {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: ChannelIndex) -> Result<T, FeagiDataProcessingError>;

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<ChannelIndex>) -> Result<Vec<T>, FeagiDataProcessingError> {
        let mut output: Vec<T> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(neuron_data, channel)?);
        };
        Ok(output)
    }
}