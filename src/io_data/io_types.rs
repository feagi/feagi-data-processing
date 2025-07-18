use std::cmp::PartialEq;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{BoundedF32, ImageFrame, Normalized0To1F32, NormalizedM1To1F32, SegmentedImageFrame};

// TODO turn all this redundant code into a Macro

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOTypeVariant {
    NormalizedM1to1F32,
    Normalized0to1F32,
    BoundedF32,
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
            IOTypeVariant::NormalizedM1to1F32 => write!(f, "Linear -1 - 1 normalized"),
            IOTypeVariant::Normalized0to1F32 => write!(f, "Linear 0 - 1 normalized"),
            IOTypeVariant::BoundedF32 => write!(f, "Bounded F32"),
            IOTypeVariant::ImageFrame => write!(f, "Image frame"),
            IOTypeVariant::SegmentedImageFrame => write!(f, "Segmented image frame"),
        }
    }
}


#[derive(Debug, Clone)]
pub enum IOTypeData
{
    LinearM1to1NormalizedF32(NormalizedM1To1F32),
    Linear0to1NormalizedF32(Normalized0To1F32),
    BoundedF32(BoundedF32),
    ImageFrame(ImageFrame),
    SegmentedImageFrame(SegmentedImageFrame),
}

impl From<NormalizedM1To1F32> for IOTypeData {
    fn from(value: NormalizedM1To1F32) -> Self {
        IOTypeData::LinearM1to1NormalizedF32(value)
    }
}

impl From<ImageFrame> for IOTypeData {
    fn from(value: ImageFrame) -> Self {
        IOTypeData::ImageFrame(value)
    }
}

impl TryFrom<IOTypeData> for NormalizedM1To1F32 {
    type Error = FeagiDataProcessingError;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::LinearM1to1NormalizedF32(float) => Ok(float),
            _ => Err(IODataError::InvalidParameters("This variable is not a -1 - 1 Linear Normalized F32!".into()).into()),
        }
    }
}

impl TryFrom<&IOTypeData> for NormalizedM1To1F32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: &IOTypeData) -> Result<Self, Self::Error> {
        match value { 
            IOTypeData::LinearM1to1NormalizedF32(float) => Ok(*float),
            _ => Err(IODataError::InvalidParameters("This variable is not a -1 - 1 Linear Normalized F32!".into()).into()),
        }
    }
}

impl TryFrom<IOTypeData> for Normalized0To1F32 {
    type Error = FeagiDataProcessingError;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::Linear0to1NormalizedF32(float) => Ok(float),
            _ => Err(IODataError::InvalidParameters("This variable is not a 0 - 1 Linear Normalized F32!".into()).into()),
        }
    }
}

impl TryFrom<&IOTypeData> for Normalized0To1F32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: &IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::Linear0to1NormalizedF32(float) => Ok(*float),
            _ => Err(IODataError::InvalidParameters("This variable is not a 0 - 1 Linear Normalized F32!".into()).into()),
        }
    }
}

impl TryFrom<IOTypeData> for BoundedF32 {
    type Error = FeagiDataProcessingError;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::BoundedF32(float) => Ok(float),
            _ => Err(IODataError::InvalidParameters("This variable is not a Bound F32!".into()).into()),
        }
    }
}

impl TryFrom<&IOTypeData> for BoundedF32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: &IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::BoundedF32(float) => Ok(*float),
            _ => Err(IODataError::InvalidParameters("This variable is not a Bound F32!".into()).into()),
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
            IOTypeData::LinearM1to1NormalizedF32(_) => IOTypeVariant::NormalizedM1to1F32,
            IOTypeData::Linear0to1NormalizedF32(_) => IOTypeVariant::Normalized0to1F32,
            IOTypeData::BoundedF32(_) => IOTypeVariant::BoundedF32,
            IOTypeData::ImageFrame(_) => IOTypeVariant::ImageFrame,
            IOTypeData::SegmentedImageFrame(_) => IOTypeVariant::SegmentedImageFrame,
        }
    }
}

