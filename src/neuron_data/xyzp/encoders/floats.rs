use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError, NeuronError};
use crate::neuron_data::translator_base_traits::{NeuronXYZPEncoder};
use crate::neuron_data::{NeuronXYZP, NeuronXYZPArrays};
use crate::io_data::LinearNormalizedF32;
use crate::genomic_structures::CorticalIOChannelIndex;
use crate::genomic_structures::CorticalDimensions;
use crate::io_cache::ChannelIndex;
use crate::neuron_data::neuron_layouts::FloatNeuronLayoutType;

pub struct FloatNeuronXYZPEncoder {
    translator_type: FloatNeuronLayoutType,
    cortical_dimensions: CorticalDimensions,
    channel_count: u32
}

impl NeuronXYZPEncoder<LinearNormalizedF32> for FloatNeuronXYZPEncoder {
    fn write_neuron_data_single_channel(&self, value: LinearNormalizedF32, target_to_overwrite: &mut NeuronXYZPArrays, channel: ChannelIndex) -> Result<(), FeagiDataProcessingError> {
        if *channel > self.channel_count {
            return Err(FeagiDataProcessingError::from(NeuronError::UnableToGenerateNeuronData(format!("Requested channel {} is not supported when max channel is {}!", channel, self.channel_count))));
        }

        match self.translator_type {
            FloatNeuronLayoutType::PSPBidirectional => {
                target_to_overwrite.expand_to_new_max_count_if_required(1);
                target_to_overwrite.reset_indexes();
                let channel_offset: u32 = FloatNeuronLayoutType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL * (channel.index() as u32) + {if value.is_sign_positive() { 1 } else { 0 }};
                let neuron: NeuronXYZP = NeuronXYZP::new(
                    channel_offset,
                    0,
                    0,
                    value.asf32()
                );
                target_to_overwrite.add_neuron(&neuron);
                return Ok(());
            },
            FloatNeuronLayoutType::SplitSignDivided => {

                // TODO Right now we are using the same algo as PSPBidirectional which works, but wouldn't it look nicer to use something that uses the full bounds?
                target_to_overwrite.expand_to_new_max_count_if_required(1);
                target_to_overwrite.reset_indexes();
                let channel_offset: u32 = FloatNeuronLayoutType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL * (channel.index() as u32) + {if value.is_sign_positive() { 1 } else { 0 }};
                let neuron: NeuronXYZP = NeuronXYZP::new(
                    channel_offset,
                    0,
                    0,
                    value.asf32()
                );
                target_to_overwrite.add_neuron(&neuron);
                return Ok(());

            },
            FloatNeuronLayoutType::Linear => {
                Err(FeagiDataProcessingError::NotImplemented)
            }
        }
    }
}

impl FloatNeuronXYZPEncoder {
    pub fn new(translator_type: FloatNeuronLayoutType, number_channels: u32, resolution_depth: usize) -> Result<Self, FeagiDataProcessingError> {
        let cortical_dimensions = translator_type.create_dimensions_for_translator_type(number_channels, resolution_depth)?;
        Ok(FloatNeuronXYZPEncoder {
            translator_type,
            cortical_dimensions,
            channel_count: number_channels
        })
    }
}