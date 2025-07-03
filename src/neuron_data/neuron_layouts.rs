use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::CorticalAreaDimensions;

pub enum FloatNeuronLayoutType {
    PSPBidirectional,
    SplitSignDivided,
    Linear,
}

impl FloatNeuronLayoutType {
    pub const CHANNEL_WIDTH_PSP_BIDIRECTIONAL: u32 = 2;
    pub const CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED: u32 = 2;
    pub const CHANNEL_WIDTH_LINEAR: u32 = 1;

    pub fn create_dimensions_for_translator_type(&self, number_channels: u32, resolution_depth: usize) -> Result<CorticalAreaDimensions, FeagiDataProcessingError> {
        if number_channels == 0 {
            return Err(IODataError::InvalidParameters("Cannot create cortical dimensions with 0 channels!".into()).into());
        }
        if resolution_depth == 0 {
            return Err(IODataError::InvalidParameters("Cannot create cortical dimensions with a resolution depth of 0!".into()).into());
        }

        match self {
            FloatNeuronLayoutType::PSPBidirectional => {
                CorticalAreaDimensions::new(number_channels  * Self::CHANNEL_WIDTH_PSP_BIDIRECTIONAL, 1, 1) // There is no resolution depth here
            }
            FloatNeuronLayoutType::SplitSignDivided => {
                CorticalAreaDimensions::new(number_channels * Self::CHANNEL_WIDTH_SPLIT_SIGN_DIVIDED, 1, resolution_depth as u32)
            }
            FloatNeuronLayoutType::Linear => {
                CorticalAreaDimensions::new(number_channels * Self::CHANNEL_WIDTH_LINEAR, 1, resolution_depth as u32 * 2 + 1)
            }
        }
    }
}