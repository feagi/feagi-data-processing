use std::collections::HashMap;
use crate::data_types::neuron_data::NeuronXYZPArrays;
use crate::error::DataProcessingError;
use crate::io_cache::ChannelIndex;

pub mod floats;

pub trait NeuronTranslator<T> {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: ChannelIndex) -> Result<T, DataProcessingError>;

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<ChannelIndex>) -> Result<Vec<T>, DataProcessingError>;
    
    fn write_neuron_data_single_channel(&self, value: T, target_to_overwrite: &mut NeuronXYZPArrays, channel: ChannelIndex) -> Result<(), DataProcessingError>;

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<ChannelIndex, T>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), DataProcessingError>;
}