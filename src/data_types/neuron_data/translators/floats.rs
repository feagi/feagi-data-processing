use std::collections::HashMap;
use crate::data_types::neuron_data::{NeuronXYZP, NeuronXYZPArrays};
use crate::data_types::RangedNormalizedF32;
use crate::error::DataProcessingError;
use crate::genome_definitions::CorticalDimensions;
use crate::io_cache::ChannelIndex;
use super::NeuronTranslator;

pub enum FloatNeuronXYZPTranslatorType {
    PSPBidirectional,
    SplitSignDivided,
    Linear,
}

impl FloatNeuronXYZPTranslatorType {
    pub const CHANNEL_WIDTH_PSP_BIDIRECTIONAL: u32 = 2;
    pub const CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED: u32 = 2;
    pub const CHANNEL_WIDTH_LINEAR: u32 = 1;
    
    pub fn create_dimensions_for_translator_type(&self, number_channels: usize, resolution_depth: usize) -> Result<CorticalDimensions, DataProcessingError> {
        if number_channels == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Cannot create cortical dimensions with 0 channels!".into()));
        }
        if resolution_depth == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Cannot create cortical dimensions with a resolution depth of 0!".into()));
        }
        
        match self {
            FloatNeuronXYZPTranslatorType::PSPBidirectional => {
                CorticalDimensions::new(number_channels as u32 * Self::CHANNEL_WIDTH_PSP_BIDIRECTIONAL, 1, 1) // There is no resolution depth here
            }
            FloatNeuronXYZPTranslatorType::SplitSignDivided => {
                CorticalDimensions::new(number_channels as u32 * Self::CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED, 1, resolution_depth as u32)
            }
            FloatNeuronXYZPTranslatorType::Linear => {
                CorticalDimensions::new(number_channels as u32 * Self::CHANNEL_WIDTH_LINEAR, 1, resolution_depth as u32 * 2 + 1)
            }
        }
    }
}

pub struct FloatNeuronXYZPTranslator {
    translator_type: FloatNeuronXYZPTranslatorType,
    cortical_dimensions: CorticalDimensions,
    channel_count: usize
}

impl NeuronTranslator<RangedNormalizedF32> for FloatNeuronXYZPTranslator {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: ChannelIndex) -> Result<RangedNormalizedF32, DataProcessingError> {
        if channel.index() > self.channel_count {
            return Err(DataProcessingError::InvalidInputBounds(format!("Requested channel {} is not supported when max channel is {}!", channel, self.channel_count)));
        }

        if neuron_data.is_empty() {
            return Ok(RangedNormalizedF32::new_zero());
        }

        let cortical_depth: f32 = self.cortical_dimensions.z as f32;

        // NOTE: The IDE for some reason thinks many branch arms are dead. Not sure why
        match self.translator_type {
            #[allow(unused_variables)] // Rust Rover seems to be blind
            FloatNeuronXYZPTranslatorType::PSPBidirectional => {
                let positive_x_index: u32 = channel.index() as u32 * FloatNeuronXYZPTranslatorType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL;
                let negative_x_index: u32 = positive_x_index + 1;
                let mut output: f32 = 0.0;
                // TODO stop going after 2?

                for neuron in neuron_data.iter() {
                    match neuron.x {
                        positive_x_index => {
                            output += neuron.p;
                        }
                        negative_x_index => {
                            output -= neuron.p;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                Ok(RangedNormalizedF32::new_with_clamp(output)?)
            }
            #[allow(unused_variables)] // Rust Rover seems to be blind
            FloatNeuronXYZPTranslatorType::SplitSignDivided => {
                let positive_x_index: u32 = channel.index() as u32 * FloatNeuronXYZPTranslatorType::CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED;
                let negative_x_index: u32 = positive_x_index + 1;
                let mut output: f32 = 0.0;
                let mut channel_neuron_count: usize = 0;
                for neuron in neuron_data.iter() {
                    match neuron.x {
                        positive_x_index => {
                            channel_neuron_count += 1;
                            let activation_feagi_index: f32 = (neuron.z + 1) as f32;
                            output += neuron.p * activation_feagi_index / cortical_depth;
                        }
                        negative_x_index => {
                            channel_neuron_count += 1;
                            let activation_feagi_index: f32 = (neuron.z + 1) as f32;
                            output -= neuron.p * activation_feagi_index / cortical_depth;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                output /= channel_neuron_count as f32;
                Ok(RangedNormalizedF32::new_with_clamp(output)?)
            }
            FloatNeuronXYZPTranslatorType::Linear => {
                Err(DataProcessingError::NotImplemented) // TODO
            }
        }
    }

    fn read_neuron_data_multi_channel(&self, neuron_data: &NeuronXYZPArrays, channels: Vec<ChannelIndex>) -> Result<Vec<RangedNormalizedF32>, DataProcessingError> {
        let mut output: Vec<RangedNormalizedF32> = Vec::with_capacity(channels.len());
        for channel in channels.iter() {
            output.push(FloatNeuronXYZPTranslator::read_neuron_data_single_channel(self, neuron_data, channel.clone())?);
        };
        Ok(output)
    }

    fn write_neuron_data_single_channel(&self, value: RangedNormalizedF32, target_to_overwrite: &mut NeuronXYZPArrays, channel: ChannelIndex) -> Result<(), DataProcessingError> {

        if channel.index() > self.channel_count {
            return Err(DataProcessingError::InvalidInputBounds(format!("Requested channel is not supported when max channel is {}!", channel)));
        }

        match self.translator_type {
            FloatNeuronXYZPTranslatorType::PSPBidirectional => {
                target_to_overwrite.expand_to_new_max_count_if_required(1);
                target_to_overwrite.reset_indexes();
                let channel_offset: u32 = FloatNeuronXYZPTranslatorType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL * (channel.index() as u32) + {if value.is_sign_positive() { 1 } else { 0 }};
                let neuron: NeuronXYZP = NeuronXYZP::new(
                    channel_offset,
                    0,
                    0,
                    value.asf32()
                );
                target_to_overwrite.add_neuron(&neuron);
                return Ok(());
            },
            FloatNeuronXYZPTranslatorType::SplitSignDivided => {

                // TODO Right now we are using the same algo as PSPBidirectional which works, but wouldn't it look nicer to use something that uses the full bounds?
                target_to_overwrite.expand_to_new_max_count_if_required(1);
                target_to_overwrite.reset_indexes();
                let channel_offset: u32 = FloatNeuronXYZPTranslatorType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL * (channel.index() as u32) + {if value.is_sign_positive() { 1 } else { 0 }};
                let neuron: NeuronXYZP = NeuronXYZP::new(
                    channel_offset,
                    0,
                    0,
                    value.asf32()
                );
                target_to_overwrite.add_neuron(&neuron);
                return Ok(());

            },
            FloatNeuronXYZPTranslatorType::Linear => {
                Err(DataProcessingError::NotImplemented)
            }
        }
    }

    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<ChannelIndex, RangedNormalizedF32>, target_to_overwrite: &mut NeuronXYZPArrays) -> Result<(), DataProcessingError> {
        for (channel, value) in channels_and_values.iter() {
            self.write_neuron_data_single_channel(*value, target_to_overwrite, channel.clone())?;
        };
        Ok(())
    }
}

impl FloatNeuronXYZPTranslator {
    pub fn new(translator_type: FloatNeuronXYZPTranslatorType, number_channels: usize, resolution_depth: usize) -> Result<Self, DataProcessingError> {
        let cortical_dimensions = translator_type.create_dimensions_for_translator_type(number_channels, resolution_depth)?;
        Ok(FloatNeuronXYZPTranslator {
            translator_type,
            cortical_dimensions,
            channel_count: number_channels
        })
    }
    

    
}
