use std::collections::HashMap;
use crate::data_types::ImageFrame;
use crate::data_types::neuron_data::NeuronXYZPArrays;
use crate::error::DataProcessingError;
use crate::genome_definitions::CorticalDimensions;
use crate::io_cache::ChannelIndex;
use super::NeuronTranslator;


pub struct ImageFrameXYZPTranslator {
    cortical_dimensions: CorticalDimensions,
    channel_count: usize
}

impl NeuronTranslator<ImageFrame> for ImageFrameXYZPTranslator {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: ChannelIndex) -> Result<ImageFrame, DataProcessingError> {
        Err(DataProcessingError::NotImplemented)
    }

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<ChannelIndex>) -> Result<Vec<ImageFrame>, DataProcessingError> {
        Err(DataProcessingError::NotImplemented)
    }

    fn write_neuron_data_single_channel(&self, value: ImageFrame, target_to_overwrite: &mut NeuronXYZPArrays, channel: ChannelIndex) -> Result<(), DataProcessingError> {
        todo!()
    }

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<ChannelIndex, ImageFrame>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), DataProcessingError> {
        todo!()
    }
}

