use std::fmt::{Display, Formatter};
use crate::error::FeagiDataProcessingError;
use crate::io_data::{IOTypeData, IOTypeVariant};
use super::super::StreamCacheProcessor;

pub struct IdentityLinearFloatCacheProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityLinearFloatCacheProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity Linear Float processor value: {:?}", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityLinearFloatCacheProcessor {
    fn get_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::LinearNormalizedFloat
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}

pub struct IdentityImageCacheProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityImageCacheProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity Image Frame processor value: {:?}", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityImageCacheProcessor {
    fn get_data_type(&self) -> IOTypeVariant {
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