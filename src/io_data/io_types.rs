use std::cmp::PartialEq;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{ImageFrame, SegmentedImageFrame};

//region IOTypeVariant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOTypeVariant {
    F32,
    F32Normalized0To1,
    F32NormalizedM1To1,
    ImageFrame,
    SegmentedImageFrame,
}

impl IOTypeVariant {
    pub fn is_of(&self, io_type: &IOTypeData) -> bool { 
        IOTypeVariant::from(io_type) == *self 
    }
}

impl std::fmt::Display for IOTypeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            IOTypeVariant::F32 => write!(f, "F32"),
            IOTypeVariant::F32Normalized0To1 => write!(f, "F32 [Normalized 0<->1]"),
            IOTypeVariant::F32NormalizedM1To1 => write!(f, "F32 [Normalized -1<->1]"),
            IOTypeVariant::ImageFrame => write!(f, "Image Frame"),
            IOTypeVariant::SegmentedImageFrame => write!(f, "Segmented Image Frame"),
        }
    }
}

impl From<IOTypeData> for IOTypeVariant {
    fn from(io_type: IOTypeData) -> Self {
        match io_type { 
            IOTypeData::F32(_) => IOTypeVariant::F32,
            IOTypeData::F32Normalized0To1(_) => IOTypeVariant::F32Normalized0To1,
            IOTypeData::F32NormalizedM1To1(_) => IOTypeVariant::F32NormalizedM1To1,
            IOTypeData::ImageFrame(_) => IOTypeVariant::ImageFrame,
            IOTypeData::SegmentedImageFrame(_) => IOTypeVariant::SegmentedImageFrame,
        }
    }
}

impl From<&IOTypeData> for IOTypeVariant {
    fn from(io_type: &IOTypeData) -> Self {
        match io_type {
            IOTypeData::F32(_) => IOTypeVariant::F32,
            IOTypeData::F32Normalized0To1(_) => IOTypeVariant::F32Normalized0To1,
            IOTypeData::F32NormalizedM1To1(_) => IOTypeVariant::F32NormalizedM1To1,
            IOTypeData::ImageFrame(_) => IOTypeVariant::ImageFrame,
            IOTypeData::SegmentedImageFrame(_) => IOTypeVariant::SegmentedImageFrame,
        }
    }
}

//endregion

//region IOTypeData
#[derive(Debug, Clone)]
pub enum IOTypeData
{
    F32(f32),
    F32Normalized0To1(f32),
    F32NormalizedM1To1(f32),
    ImageFrame(ImageFrame),
    SegmentedImageFrame(SegmentedImageFrame),
}

impl std::fmt::Display for IOTypeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            IOTypeData::F32(float) => write!(f, "IOTypeData(f32({}))", float),
            IOTypeData::F32Normalized0To1(float) => write!(f, "IOTypeData(f32[Normalized 0<->1]({}))", float),
            IOTypeData::F32NormalizedM1To1(float) => write!(f, "IOTypeData(f32[Normalized -1<->1]({}))", float),
            IOTypeData::ImageFrame(frame) => write!(f, "IOTypeData({})", frame),
            IOTypeData::SegmentedImageFrame(frame) => write!(f, "IOTypeData({})", frame),
        }
    }
}

// NOTE: Not implementing "From<f32> for IOTypeData" since there are multiple paths

impl From<ImageFrame> for IOTypeData {
    fn from(value: ImageFrame) -> Self {
        IOTypeData::ImageFrame(value)
    }
}

impl From<SegmentedImageFrame> for IOTypeData {
    fn from(value: SegmentedImageFrame) -> Self {
        IOTypeData::SegmentedImageFrame(value)
    }
}

impl TryFrom<IOTypeData> for f32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value { 
            IOTypeData::F32(float) => Ok(float),
            IOTypeData::F32Normalized0To1(float) => Ok(float),
            IOTypeData::F32NormalizedM1To1(float) => Ok(float),
            _ => Err(IODataError::InvalidParameters("This variable is not a f32 type value!".into()).into()),
        }
    }
}

impl TryFrom<&IOTypeData> for f32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: &IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::F32(float) => Ok(*float),
            IOTypeData::F32Normalized0To1(float) => Ok(*float),
            IOTypeData::F32NormalizedM1To1(float) => Ok(*float),
            _ => Err(IODataError::InvalidParameters("This variable is not a f32 type value!".into()).into()),
        }
    }
}


impl TryFrom<IOTypeData> for ImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image) => Ok(image),
            _ => Err(IODataError::InvalidParameters("This variable is not a Image Frame!".into()).into()),
        }
    }
}

impl<'a> TryFrom<&'a IOTypeData> for &'a ImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: &'a IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a Image Frame!".into()).into()),
        }
    }
}

impl<'a> TryFrom<&'a mut IOTypeData> for &'a mut ImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: &'a mut IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a Image Frame!".into()).into()),
        }
    }
}

impl IOTypeData {
    pub fn new_f32(value: f32) -> Result<Self, FeagiDataProcessingError> {
        if value.is_nan() || value.is_infinite() {
            return Err(IODataError::InvalidParameters("Input value cannot be NaN or Infinite!".into()).into());
        }
        Ok(Self::F32(value))
    }
    
    pub fn new_0_1_f32(value: f32) -> Result<Self, FeagiDataProcessingError> {
        if value.is_nan() || value.is_infinite() {
            return Err(IODataError::InvalidParameters("Input value cannot be NaN or Infinite!".into()).into());
        }
        if value < 0.0 || value > 1.0 {
            return Err(IODataError::InvalidParameters("Input value must be between 0 and 1!".into()).into());
        }
        Ok(Self::F32Normalized0To1(value))
    }

    pub fn new_m1_1_f32(value: f32) -> Result<Self, FeagiDataProcessingError> {
        if value.is_nan() || value.is_infinite() {
            return Err(IODataError::InvalidParameters("Input value cannot be NaN or Infinite!".into()).into());
        }
        if value < -1.0 || value > 1.0 {
            return Err(IODataError::InvalidParameters("Input value must be between -1 and 1!".into()).into());
        }
        Ok(Self::F32NormalizedM1To1(value))
    }
}

//endregion
