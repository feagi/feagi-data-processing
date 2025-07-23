use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{CorticalID, CorticalType, SingleChannelDimensions};
use crate::neuron_data::xyzp::coders::{NeuronXYZPEncoder};
use crate::neuron_data::xyzp::coders::encoders::{ImageFrameNeuronXYZPEncoder, NormalizedM1To1F32PSPBirdirectionalNeuronXYZPEncoder, NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder, Normalized0To1F32LinearNeuronXYZPEncoder};

pub enum NeuronCoderVariantType {
    NormalizedM1To1F32_SplitSignDivided,
    NormalizedM1To1F32_PSPBirdirectionalDivided,
    Normalized0To1F32,
    ImageFrame,
    SegmentedImageFrame,
}

pub fn instantiate_encoder_by_type(neuron_coder_type: NeuronCoderVariantType, cortical_ids_targeted: &[CorticalID], channel_dimensions: SingleChannelDimensions)
    -> Result<Box<dyn NeuronXYZPEncoder + Sync + Send>, FeagiDataProcessingError> {
    
    const STANDARD_EXPECTED_CORTICAL_ID_COUNT: usize = 1;
    
    match neuron_coder_type {
        NeuronCoderVariantType::NormalizedM1To1F32_SplitSignDivided => {
            verify_number_cortical_IDs_sensible(cortical_ids_targeted, STANDARD_EXPECTED_CORTICAL_ID_COUNT);
            Ok(Box::new(NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::NormalizedM1To1F32_PSPBirdirectionalDivided => {
            verify_number_cortical_IDs_sensible(cortical_ids_targeted, STANDARD_EXPECTED_CORTICAL_ID_COUNT);
            Ok(Box::new(NormalizedM1To1F32PSPBirdirectionalNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::Normalized0To1F32 => {
            verify_number_cortical_IDs_sensible(cortical_ids_targeted, STANDARD_EXPECTED_CORTICAL_ID_COUNT);
            Ok(Box::new(Normalized0To1F32LinearNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::ImageFrame => {
            verify_number_cortical_IDs_sensible(cortical_ids_targeted, STANDARD_EXPECTED_CORTICAL_ID_COUNT);
            Ok(Box::new(ImageFrameNeuronXYZPEncoder::new(cortical_ids_targeted.clone()[0], channel_dimensions)))
        }
        NeuronCoderVariantType::SegmentedImageFrame => {
            const NUMBER_SEGMENTS_IN_SEGMENTED_FRAME: usize = 9;
            verify_number_cortical_IDs_sensible(cortical_ids_targeted, NUMBER_SEGMENTS_IN_SEGMENTED_FRAME);
            Err(FeagiDataProcessingError::NotImplemented) // TODO
        }
    }
}

fn verify_number_cortical_IDs_sensible(cortical_ids: &[CorticalID], expected_count: usize) -> Result<(), FeagiDataProcessingError> {
    if cortical_ids.len() == 0 {
        return Err(FeagiDataProcessingError::InternalError("Cannot instantiate a coder with 0 cortical IDs!".into()))
    }
    if expected_count != cortical_ids.len() {
        return Err(FeagiDataProcessingError::InternalError(format!("Expected {} cortical IDs, but given {}!!", expected_count,cortical_ids.len())))
    }
    Ok(())
}