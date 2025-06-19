use crate::data_types::neuron_data::NeuronXYZPArrays;
use crate::error::DataProcessingError;
use crate::genome_definitions::CorticalDimensions;

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

impl FloatNeuronXYZPTranslator {
    pub fn new(translator_type: FloatNeuronXYZPTranslatorType, number_channels: usize, resolution_depth: usize) -> Result<Self, DataProcessingError> {
        let cortical_dimensions = translator_type.create_dimensions_for_translator_type(number_channels, resolution_depth)?;
        Ok(FloatNeuronXYZPTranslator {
            translator_type,
            cortical_dimensions,
            channel_count: number_channels
        })
    }
    
    pub fn calculate_nonscaled_float(&self, neuron_data: NeuronXYZPArrays, channel: usize) -> Result<f32, DataProcessingError> {
        if channel > self.channel_count {
            return Err(DataProcessingError::InvalidInputBounds(format!("Requested channel {} is not supported when max channel is {}!", channel, self.channel_count)));
        }

        if neuron_data.is_empty() {
            return Ok(0.0);
        }

        let cortical_depth: f32 = self.cortical_dimensions.z as f32;
        
        match self.translator_type {
            FloatNeuronXYZPTranslatorType::PSPBidirectional => {
                let positive_x_index: u32 = channel as u32 * FloatNeuronXYZPTranslatorType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL;
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
                Ok(output.clamp(-1.0, 1.0))

            }
            FloatNeuronXYZPTranslatorType::SplitSignDivided => {
                let positive_x_index: u32 = channel as u32 * FloatNeuronXYZPTranslatorType::CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED;
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
                Ok(output.clamp(-1.0, 1.0))
            }
            FloatNeuronXYZPTranslatorType::Linear => {
                Err(DataProcessingError::NotImplemented)
            }
        }
    }
    
}
