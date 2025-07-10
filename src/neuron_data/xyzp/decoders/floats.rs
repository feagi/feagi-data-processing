use crate::error::{FeagiDataProcessingError, NeuronError};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPDecoder};
use crate::neuron_data::xyzp::NeuronXYZPArrays;
use crate::io_data::{IOTypeData, IOTypeVariant, LinearNormalizedF32};
use crate::genomic_structures::{CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::genomic_structures::CorticalAreaDimensions;
use crate::neuron_data::neuron_layouts::FloatNeuronLayoutType;

// TODO use enum_dispatch to make this look less cancer

pub struct LinearNormalizedFloatNeuronXYZPDecoder {
    single_cortical_id: [CorticalID; 1],
    translator_type: FloatNeuronLayoutType,
    cortical_dimensions: CorticalAreaDimensions,
    channel_count: u32
}

impl NeuronXYZPDecoder for LinearNormalizedFloatNeuronXYZPDecoder {
    fn get_data_type() -> IOTypeVariant {
        IOTypeVariant::LinearNormalizedFloat
    }

    fn get_cortical_ids_reading_from(&self) -> &[CorticalID] {
        &self.single_cortical_id
    }
    
    fn read_neuron_data_single_channel(&self, cortical_channel: CorticalIOChannelIndex, neuron_data: &CorticalMappedXYZPNeuronData) -> Result<IOTypeData, FeagiDataProcessingError> {

        if *cortical_channel > self.channel_count {
            return Err(FeagiDataProcessingError::from(NeuronError::UnableToGenerateNeuronData(format!("Requested channel {} is not supported when max channel is {}!", *cortical_channel, self.channel_count))).into());
        }
        
        let cortical_id = self.single_cortical_id[0];
        if !neuron_data.contains(&cortical_id) {
            return Ok(LinearNormalizedF32::new_zero().into());
        }
        
        let neuron_data: &NeuronXYZPArrays = neuron_data.borrow(&cortical_id).unwrap();

        if neuron_data.is_empty() {
            return Ok(LinearNormalizedF32::new_zero().into());
        }

        let cortical_depth: f32 = self.cortical_dimensions.z as f32;

        // NOTE: The IDE for some reason thinks many branch arms are dead. Not sure why
        match self.translator_type {
            #[allow(unused_variables)] // Rust Rover seems to be blind
            FloatNeuronLayoutType::PSPBidirectional => {
                let positive_x_index: u32 = *cortical_channel * FloatNeuronLayoutType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL;
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
                Ok(LinearNormalizedF32::new_with_clamp(output)?.into())
            }
            #[allow(unused_variables)] // Rust Rover seems to be blind
            FloatNeuronLayoutType::SplitSignDivided => {
                let positive_x_index: u32 = *cortical_channel * FloatNeuronLayoutType::CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED;
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
                Ok(LinearNormalizedF32::new_with_clamp(output)?.into())
            }
            FloatNeuronLayoutType::Linear => {
                Err(FeagiDataProcessingError::NotImplemented) // TODO
            }
            
            
        }
    }
}

impl LinearNormalizedFloatNeuronXYZPDecoder {
    pub fn new(number_channels: u32, cortical_type: CorticalType, cortical_index: CorticalGroupingIndex, translator_type: FloatNeuronLayoutType, resolution_depth: usize) -> Result<Self, FeagiDataProcessingError> {
        cortical_type.verify_valid_io_variant(&Self::get_data_type())?;
        let cortical_id = CorticalID::try_from_cortical_type(&cortical_type, cortical_index)?;
        let cortical_dimensions = translator_type.create_dimensions_for_translator_type(number_channels, resolution_depth)?;
        
        Ok(LinearNormalizedFloatNeuronXYZPDecoder {
            single_cortical_id: [cortical_id],
            translator_type: translator_type,
            cortical_dimensions: cortical_dimensions,
            channel_count: number_channels
        })
    }
}