use crate::error::{FeagiDataProcessingError, NeuronError};
use crate::neuron_data::xyzp::NeuronXYZPDecoder;
use crate::neuron_data::NeuronXYZPArrays;
use crate::io_data::LinearNormalizedF32;
use crate::genomic_structures::CorticalIOChannelIndex;
use crate::genomic_structures::CorticalAreaDimensions;
use crate::neuron_data::neuron_layouts::FloatNeuronLayoutType;

pub struct FloatNeuronXYZPDecoder {
    translator_type: FloatNeuronLayoutType,
    cortical_dimensions: CorticalAreaDimensions,
    channel_count: u32
}

impl NeuronXYZPDecoder<LinearNormalizedF32> for FloatNeuronXYZPDecoder {
    fn read_neuron_data_single_channel(&self, neuron_data: &NeuronXYZPArrays, channel: CorticalIOChannelIndex) -> Result<LinearNormalizedF32, FeagiDataProcessingError> {
        if *channel > self.channel_count {
            return Err(FeagiDataProcessingError::from(NeuronError::UnableToGenerateNeuronData(format!("Requested channel {} is not supported when max channel is {}!", channel, self.channel_count))));
        }

        if neuron_data.is_empty() {
            return Ok(LinearNormalizedF32::new_zero());
        }

        let cortical_depth: f32 = self.cortical_dimensions.z as f32;

        // NOTE: The IDE for some reason thinks many branch arms are dead. Not sure why
        match self.translator_type {
            #[allow(unused_variables)] // Rust Rover seems to be blind
            FloatNeuronLayoutType::PSPBidirectional => {
                let positive_x_index: u32 = *channel * FloatNeuronLayoutType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL;
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
                Ok(LinearNormalizedF32::new_with_clamp(output)?)
            }
            #[allow(unused_variables)] // Rust Rover seems to be blind
            FloatNeuronLayoutType::SplitSignDivided => {
                let positive_x_index: u32 = *channel * FloatNeuronLayoutType::CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED;
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
                Ok(LinearNormalizedF32::new_with_clamp(output)?)
            }
            FloatNeuronLayoutType::Linear => {
                Err(FeagiDataProcessingError::NotImplemented) // TODO
            }
        }
    }
}

impl FloatNeuronXYZPDecoder {
    pub fn new(translator_type: FloatNeuronLayoutType, number_channels: u32, resolution_depth: usize) -> Result<Self, FeagiDataProcessingError> {
        let cortical_dimensions = translator_type.create_dimensions_for_translator_type(number_channels, resolution_depth)?;
        Ok(FloatNeuronXYZPDecoder {
            translator_type,
            cortical_dimensions,
            channel_count: number_channels
        })
    }
}