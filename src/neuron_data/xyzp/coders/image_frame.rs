use std::collections::HashMap;
use crate::io_data::ImageFrame;
use crate::neuron_data::NeuronXYZPArrays;
use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::CorticalDimensions;
use crate::io_cache::ChannelIndex;

pub struct ImageFrameXYZPTranslator {
    cortical_dimensions: CorticalDimensions,
    channel_count: usize
}

impl NeuronTranslator<ImageFrame> for ImageFrameXYZPTranslator {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: ChannelIndex) -> Result<ImageFrame, FeagiDataProcessingError> {
        Err(DataProcessingError::NotImplemented)
    }

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<ChannelIndex>) -> Result<Vec<ImageFrame>, FeagiDataProcessingError> {
        Err(DataProcessingError::NotImplemented)
    }

    fn write_neuron_data_single_channel(&self, value: ImageFrame, target_to_overwrite: &mut NeuronXYZPArrays, channel: ChannelIndex) -> Result<(), FeagiDataProcessingError> {
        todo!()
    }

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<ChannelIndex, ImageFrame>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError> {
        todo!()
    }
}

