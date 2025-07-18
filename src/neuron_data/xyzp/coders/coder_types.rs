use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalType, SingleChannelDimensions};
use crate::neuron_data::xyzp::{NeuronXYZPEncoder, NormalizedM1To1F32PSPBirdirectionalNeuronXYZPEncoder, NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder, Normalized0To1F32LinearNeuronXYZPEncoder, ImageFrameNeuronXYZPEncoder};

pub(crate) enum NeuronCoderVariantType {
    NormalizedM1To1F32_SplitSignDivided,
    NormalizedM1To1F32_PSPBirdirectionalDivided,
    Normalized0To1F32,
    ImageFrame,
}

pub(crate) fn instantiate_encoder_by_type(neuron_coder_type: NeuronCoderVariantType, cortical_ids_targeted: &[CorticalID], channel_dimensions: SingleChannelDimensions)
    -> Result<Box<dyn NeuronXYZPEncoder>, FeagiDataProcessingError> {
    
    match neuron_coder_type {
        NeuronCoderVariantType::NormalizedM1To1F32_SplitSignDivided => {
            Ok(Box::new(NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::NormalizedM1To1F32_PSPBirdirectionalDivided => {
            Ok(Box::new(NormalizedM1To1F32PSPBirdirectionalNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::Normalized0To1F32 => {
            Ok(Box::new(Normalized0To1F32LinearNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::ImageFrame => {
            Ok(Box::new(ImageFrameNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
    }
}