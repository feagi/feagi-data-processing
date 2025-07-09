use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::CorticalIOChannelIndex;
use crate::io_data::IOTypeData;
use super::{NeuronXYZPArrays};

// Coders can be enums since they do not store values, they merely are organizational units directing
// to specific methods for reading and writing neural data

pub trait NeuronXYZPEncoder {
    fn write_neuron_data_single_channel(&self, wrapped_value: IOTypeData, target_to_overwrite: &mut NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError>;

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, IOTypeData>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, target_to_overwrite, channel)?;
        };
        Ok(())
    }
}

pub trait NeuronXYZPDecoder {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<IOTypeData, FeagiDataProcessingError>;

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<CorticalIOChannelIndex>) -> Result<Vec<IOTypeData>, FeagiDataProcessingError> {
        let mut output: Vec<IOTypeData> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(neuron_data, channel)?);
        };
        Ok(output)
    }
}