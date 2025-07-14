use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex};
use crate::io_data::{IOTypeData, IOTypeVariant};
use super::{NeuronXYZPArrays, CorticalMappedXYZPNeuronData};

// Coders can be enums since they do not store dynamic values, they merely are organizational units directing
// to specific methods for reading and writing neural data

// TODO could we turn this into an enum via Enum_Dispatch?
// TODO right now the multi channel functions call the single functions in a loop, while this technically works, we can do better. Right now each single channel fn iterates over all neurons for some types (floats), looking just for the specific channel it is set to. We can vectorize this

pub trait NeuronXYZPEncoder {

    fn get_encoded_data_type(&self) -> IOTypeVariant;
    
    fn get_cortical_ids_writing_to(&self) -> &[CorticalID];

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError>;

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, &IOTypeData>, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, channel, write_target)?;
        };
        Ok(())
    }
}

pub trait NeuronXYZPDecoder {

    fn get_data_type(&self) -> IOTypeVariant;

    fn get_cortical_ids_reading_from(&self) -> &[CorticalID];

    fn read_neuron_data_single_channel(&self, cortical_channel: CorticalIOChannelIndex, neuron_data: &CorticalMappedXYZPNeuronData) -> Result<IOTypeData, FeagiDataProcessingError>;

    fn read_neuron_data_multi_channel(&self, neuron_data: &CorticalMappedXYZPNeuronData, channels: &[CorticalIOChannelIndex]) -> Result<Vec<IOTypeData>, FeagiDataProcessingError> {
        let mut output: Vec<IOTypeData> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(*channel, neuron_data)?);
        };
        Ok(output)
    }
}