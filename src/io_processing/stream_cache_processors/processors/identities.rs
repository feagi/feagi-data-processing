use std::fmt::{Display, Formatter};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant, ImageFrame, SegmentedImageFrame};
use crate::io_processing::StreamCacheProcessor;

#[derive(Debug, Clone)]
pub struct IdentityFloatProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityFloatProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdentityFloatProcessor({:?})", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityFloatProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }
    
    /// Process new input and 
    fn process_new_input(&mut self, value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

impl IdentityFloatProcessor {
    pub fn new(initial_value: f32) -> Result<Self, FeagiDataProcessingError> {
        if initial_value.is_nan() || initial_value.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given float {} is not valid!", initial_value)).into());
        }
        Ok(IdentityFloatProcessor{
            previous_value: IOTypeData::F32(initial_value),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IdentityImageFrameProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityImageFrameProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdentityImageFrameProcessor({:?})", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityImageFrameProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

impl IdentityImageFrameProcessor {
    pub fn new(initial_image: ImageFrame) -> Result<Self, FeagiDataProcessingError> {
        Ok(IdentityImageFrameProcessor{
            previous_value: IOTypeData::ImageFrame(initial_image),
        })
    }
}


#[derive(Debug, Clone)]
pub struct IdentitySegmentedImageFrameProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentitySegmentedImageFrameProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdentitySegmentedImageFrameProcessor({:?})", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentitySegmentedImageFrameProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::SegmentedImageFrame
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::SegmentedImageFrame
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

impl IdentitySegmentedImageFrameProcessor {
    pub fn new(initial_segmented_image: SegmentedImageFrame) -> Result<Self, FeagiDataProcessingError> {
        Ok(IdentitySegmentedImageFrameProcessor{
            previous_value: IOTypeData::SegmentedImageFrame(initial_segmented_image),
        })
    }
}

