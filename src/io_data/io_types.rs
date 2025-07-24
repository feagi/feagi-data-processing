use std::cmp::PartialEq;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{ImageFrame, SegmentedImageFrame};

// TODO turn all this redundant code into a Macro

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOTypeVariant {
    F32,
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
            IOTypeVariant::ImageFrame => write!(f, "Image frame"),
            IOTypeVariant::SegmentedImageFrame => write!(f, "Segmented image frame"),
        }
    }
}

impl From<IOTypeData> for IOTypeVariant {
    fn from(io_type: IOTypeData) -> Self {
        match io_type { 
            IOTypeData::F32(_) => IOTypeVariant::F32,
            IOTypeData::ImageFrame(_) => IOTypeVariant::ImageFrame,
            IOTypeData::SegmentedImageFrame(_) => IOTypeVariant::SegmentedImageFrame,
        }
    }
}

impl From<&IOTypeData> for IOTypeVariant {
    fn from(io_type: &IOTypeData) -> Self {
        match io_type {
            IOTypeData::F32(_) => IOTypeVariant::F32,
            IOTypeData::ImageFrame(_) => IOTypeVariant::ImageFrame,
            IOTypeData::SegmentedImageFrame(_) => IOTypeVariant::SegmentedImageFrame,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IOTypeData
{
    F32(f32),
    ImageFrame(ImageFrame),
    SegmentedImageFrame(SegmentedImageFrame),
}

impl std::fmt::Display for IOTypeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            IOTypeData::F32(float) => write!(f, "IOTypeData({})", float),
            IOTypeData::ImageFrame(frame) => write!(f, "IOTypeData({})", frame),
            IOTypeData::SegmentedImageFrame(frame) => write!(f, "IOTypeData({})", frame),
        }
    }
}

impl From<f32> for IOTypeData {
    fn from(float: f32) -> IOTypeData {
        IOTypeData::F32(float)
    }
}

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
            _ => Err(IODataError::InvalidParameters("This variable is not a f32!".into()).into()),
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
