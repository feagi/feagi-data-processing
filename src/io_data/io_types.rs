use std::cmp::PartialEq;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{ImageFrame, LinearNormalizedF32};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOTypeVariant {
    LinearNormalizedFloat,
    ImageFrame,
    SegmentedImageFrame,
}

impl IOTypeVariant {
    pub fn is_of(&self, io_type: &IOTypeData) -> bool {
        io_type.variant() == *self
    }
}

impl std::fmt::Display for IOTypeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            IOTypeVariant::LinearNormalizedFloat => write!(f, "Linear normalized"),
            IOTypeVariant::ImageFrame => write!(f, "Image frame"),
            IOTypeVariant::SegmentedImageFrame => write!(f, "Segmented image frame"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IOTypeData
{
    LinearNormalizedFloat(LinearNormalizedF32),
    ImageFrame(ImageFrame),
}

impl From<LinearNormalizedF32> for IOTypeData {
    fn from(value: LinearNormalizedF32) -> Self {
        IOTypeData::LinearNormalizedFloat(value)
    }
}

impl From<ImageFrame> for IOTypeData {
    fn from(value: ImageFrame) -> Self {
        IOTypeData::ImageFrame(value)
    }
}

impl TryFrom<IOTypeData> for LinearNormalizedF32 {
    type Error = FeagiDataProcessingError;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::LinearNormalizedFloat(float) => Ok(float),
            other => Err(IODataError::InvalidParameters("This variable is not a Linear Normalized F32!".into()).into()),
        }
    }
}

impl TryFrom<&IOTypeData> for LinearNormalizedF32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: &IOTypeData) -> Result<Self, Self::Error> {
        match value { 
            IOTypeData::LinearNormalizedFloat(float) => Ok(*float),
            _ => Err(IODataError::InvalidParameters("This variable is not a Linear Normalized F32!".into()).into()),
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
    pub fn variant(&self) -> IOTypeVariant {
        match self {
            IOTypeData::LinearNormalizedFloat(_) => IOTypeVariant::LinearNormalizedFloat,
            IOTypeData::ImageFrame(_) => IOTypeVariant::ImageFrame
        }
    }
}

