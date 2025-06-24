use crate::error::DataProcessingError;

mod image;
pub mod neuron_data;

pub use image::image_frame::ImageFrame as ImageFrame;
pub use image::segmented_vision_frame::SegmentedVisionFrame as SegmentedVisionFrame;
pub use image::descriptors as image_descriptors;


const EPSILON: f32 = 0.0001;

#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
pub struct RangedNormalizedF32 {
    float: f32
}

impl RangedNormalizedF32 {
    pub fn new(float: f32) -> Result<Self, DataProcessingError> {
        RangedNormalizedF32::validate_float(float)?;
        Ok(Self { float })
    }
    
    pub fn new_with_clamp(float: f32)  -> Result<Self, DataProcessingError> {
        if float.is_nan() {
            return Err(DataProcessingError::InvalidInputBounds("Input float may not be NaN!".into()));
        }
        let clamped = float.clamp(-1.0, 1.0);
        Ok(Self { float: clamped })
    }
    
    pub fn new_zero() -> Self {
        Self { float: 0.0 }
    }
    
    pub fn update(&mut self, new_float: f32) -> Result<(), DataProcessingError> {
        RangedNormalizedF32::validate_float(new_float)?;
        self.float = new_float;
        Ok(())
    }

    pub fn update_with_clamp(&mut self, new_float: f32) -> Result<(), DataProcessingError> {
        if new_float.is_nan() {
            return Err(DataProcessingError::InvalidInputBounds("Input float may not be NaN!".into()));
        }
        self.float = new_float.clamp(-1.0 - EPSILON, 1.0 + EPSILON);
        Ok(())
    }
    
    pub fn asf32(&self) -> f32 {
        self.float
    }
    
    pub fn is_sign_positive(&self) -> bool {
        self.float.is_sign_positive()
    }
    
    fn validate_float(float: f32) -> Result<(), DataProcessingError> {
        if float.is_nan() {
            return Err(DataProcessingError::InvalidInputBounds("Input float may not be NaN!".into()));
        }
        if float.is_infinite() {
            return Err(DataProcessingError::InvalidInputBounds("Input float may not be infinite!".into()));
        }

        if float.abs() > 1.0 + EPSILON {
            return Err(DataProcessingError::InvalidInputBounds("Input float may not be less than negative one or greater than 1!".into()));
        }
        Ok(())
    }
}