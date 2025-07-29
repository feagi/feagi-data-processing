use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalGroupingIndex, CorticalID, SingleChannelDimensions};
use crate::neuron_data::xyzp::coders::{NeuronXYZPEncoder};
use crate::neuron_data::xyzp::coders::encoders::{ImageFrameNeuronXYZPEncoder, F32PSPBidirectionalNeuronXYZPEncoder, F32SplitSignDividedNeuronXYZPEncoder, F32LinearNeuronXYZPEncoder};

pub enum NeuronEncoderVariantType { // Enum itself must be exposed (methods don't)
    F32Normalized0To1_Linear,
    F32NormalizedM1To1_PSPBidirectional,
    F32NormalizedM1To1_SplitSignDivided,
    ImageFrame,
    SegmentedImageFrame,
}

impl NeuronEncoderVariantType {
    pub(crate) fn instantiate_single_ipu_encoder(&self, cortical_id: &CorticalID, validated_channel_dimensions: &SingleChannelDimensions) // Doesn't need to be exposed out of crate
        -> Result<Box<dyn NeuronXYZPEncoder + Sync + Send>, FeagiDataProcessingError> {
        
        // Assuming channel_dimensions is validated
        
        if !cortical_id.get_cortical_type().is_type_sensor() {
            return Err(FeagiDataProcessingError::InternalError("Only IPUs can spawn encoders!".into()))
        }
        
        match self {
            NeuronEncoderVariantType::F32Normalized0To1_Linear => {
                Ok(Box::new(F32LinearNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }
            NeuronEncoderVariantType::F32NormalizedM1To1_PSPBidirectional => {
                Ok(Box::new(F32PSPBidirectionalNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }
            
            NeuronEncoderVariantType::F32NormalizedM1To1_SplitSignDivided => {
                Ok(Box::new(F32SplitSignDividedNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }
            
            NeuronEncoderVariantType::ImageFrame => {
                Ok(Box::new(ImageFrameNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }
            NeuronEncoderVariantType::SegmentedImageFrame => {
                Err(FeagiDataProcessingError::InternalError("Segmented Image Frame is not a single IPU encoder!".into()))
            }
        }
    }
    
    // TODO instantiate segmented image frame encoder, find a way to pass in the multiple dimensions
}
