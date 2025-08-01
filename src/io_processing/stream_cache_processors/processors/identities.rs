//! Identity processors that pass data through unchanged.
//!
//! This module provides "pass-through" processors that implement the StreamCacheProcessor
//! interface but don't modify the data in any way. As at least 1 processor is required when
//! adding channels, these are useful if the user does not wish to transform the data

use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant, ImageFrame, SegmentedImageFrame};
use crate::io_processing::StreamCacheProcessor;

/// A stream processor that passes float values through unchanged.
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
    
    /// Process new input and store it unchanged.
    fn process_new_input(&mut self, value: &IOTypeData, _: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

impl IdentityFloatProcessor {
    /// Creates a new IdentityFloatProcessor.
    ///
    /// # Arguments
    /// * `initial_value` - The initial float value to store (must be finite)
    ///
    /// # Returns
    /// * `Ok(IdentityFloatProcessor)` - A new processor instance
    /// * `Err(FeagiDataProcessingError)` - If initial_value is invalid (NaN/infinite)
    pub fn new(initial_value: f32) -> Result<Self, FeagiDataProcessingError> {
        if initial_value.is_nan() || initial_value.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given float {} is not valid!", initial_value)).into());
        }
        Ok(IdentityFloatProcessor{
            previous_value: IOTypeData::F32(initial_value),
        })
    }
}

/// A stream processor that passes image frames through unchanged.
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

    fn process_new_input(&mut self, value: &IOTypeData, _: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

impl IdentityImageFrameProcessor {
    /// Creates a new IdentityImageFrameProcessor.
    ///
    /// # Arguments
    /// * `initial_image` - The initial ImageFrame to store
    ///
    /// # Returns
    /// * `Ok(IdentityImageFrameProcessor)` - A new processor instance
    pub fn new(initial_image: ImageFrame) -> Result<Self, FeagiDataProcessingError> {
        Ok(IdentityImageFrameProcessor{
            previous_value: IOTypeData::ImageFrame(initial_image),
        })
    }
}


/// A stream processor that passes segmented image frames through unchanged.
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

    fn process_new_input(&mut self, value: &IOTypeData, _: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

impl IdentitySegmentedImageFrameProcessor {
    /// Creates a new IdentitySegmentedImageFrameProcessor.
    ///
    /// # Arguments
    /// * `initial_segmented_image` - The initial SegmentedImageFrame to store
    ///
    /// # Returns
    /// * `Ok(IdentitySegmentedImageFrameProcessor)` - A new processor instance
    pub fn new(initial_segmented_image: SegmentedImageFrame) -> Result<Self, FeagiDataProcessingError> {
        Ok(IdentitySegmentedImageFrameProcessor{
            previous_value: IOTypeData::SegmentedImageFrame(initial_segmented_image),
        })
    }
}

