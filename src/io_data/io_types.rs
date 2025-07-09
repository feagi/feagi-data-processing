use std::cmp::PartialEq;
use crate::io_data::{ImageFrame, LinearNormalizedF32};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOTypeVariant {
    LinearNormalizedFloat,
    ImageFrame
}

impl IOTypeVariant {
    pub fn is_of(&self, io_type: &IOTypeData) -> bool {
        io_type.variant() == *self
    }
}

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
    type Error = IOTypeData;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::LinearNormalizedFloat(float) => Ok(float),
            other => Err(other),
        }
    }
}

impl TryFrom<IOTypeData> for ImageFrame {
    type Error = IOTypeData;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image) => Ok(image),
            other => Err(other),
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

