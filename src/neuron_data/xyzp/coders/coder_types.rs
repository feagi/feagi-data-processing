use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, SingleChannelDimensions};
use crate::neuron_data::xyzp::{NeuronXYZPEncoder, NormalizedM1To1F32PSPBirdirectionalNeuronXYZPEncoder};

pub(crate) enum NeuronCoderType {
    Encoder(NeuronCoderVariantType),
    Decoder(NeuronCoderVariantType)
}

pub(crate) enum NeuronCoderVariantType {
    NormalizedM1To1F32_SplitSignDivided,
    NormalizedM1To1F32_PSPBirdirectionalDivided,
    Normalized0To1F32,
    ImageFrame,
}

pub(crate) fn instantiate_encoder_by_type(neuron_coder_type: NeuronCoderType, cortical_ids_targeted:  channel_dimensions: SingleChannelDimensions)
    -> Result<Box<dyn NeuronXYZPEncoder>, FeagiDataProcessingError> {
    match neuron_coder_type {
        NeuronCoderType::Decoder(neuron_coder_type) => {
            return Err(FeagiDataProcessingError::InternalError("Cannot use this func to instantiate a neuron decoder".to_string()))
        }
        NeuronCoderType::Encoder(neuron_coder_type) => {
            match neuron_coder_type {
                NeuronCoderVariantType::NormalizedM1To1F32_PSPBirdirectionalDivided => {
                    
                    
                    return Ok(Box<dyn NormalizedM1To1F32PSPBirdirectionalNeuronXYZPEncoder::new())
                }
                NeuronCoderVariantType::Normalized0To1F32 => {
                
                }
            }
        }
    }
}