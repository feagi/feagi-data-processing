use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::CorticalIOChannelIndex;
use super::{NeuronXYZPArrays};
use super::decoders::floats::{FloatNeuronXYZPDecoder};
use super::encoders::floats::{FloatNeuronXYZPEncoder};

// Coders can be enums since they do not store values, they merely are organizational units directing
// to specific methods for reading and writing neural data
#[enum_dispatch(NeuronXYZPEncoderControl<T>)]
pub enum NeuronXYZPEncoder<T> {
    FloatNeuronXYZPEncoder
}

#[enum_dispatch(NeuronXYZPDecoderControl<T>)]
pub enum NeuronXYZPDecoder<T> {
    FloatNeuronXYZPDecoder
}


#[enum_dispatch]
pub(crate) trait NeuronXYZPEncoderControl<T> {
    fn write_neuron_data_single_channel(&self, value: T, target_to_overwrite: &mut NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError>;

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, T>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, target_to_overwrite, channel)?;
        };
        Ok(())
    }
}

#[enum_dispatch]
pub(crate) trait NeuronXYZPDecoderControl<T> {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<T, FeagiDataProcessingError>;

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<CorticalIOChannelIndex>) -> Result<Vec<T>, FeagiDataProcessingError> {
        let mut output: Vec<T> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(neuron_data, channel)?);
        };
        Ok(output)
    }
}