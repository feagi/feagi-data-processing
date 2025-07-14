use std::fmt::{Display, Formatter};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use super::StreamCacheProcessor;

#[derive(Debug, Clone)]
pub struct IdentityImageFrameCacheProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityImageFrameCacheProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity Image Frame processor value: {:?}", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityImageFrameCacheProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        if value.variant() != self.get_input_data_type() {
            return Err(IODataError::InvalidParameters("Value for IdentityImageFrameCacheProcessor must be a ImageFrame!".into()).into());
        }
        self.previous_value = value;
        Ok(&self.previous_value) // No Change!
    }
}

impl IdentityImageFrameCacheProcessor {
    pub fn new(initial_value: IOTypeData) -> Result<Self, FeagiDataProcessingError> {
        if initial_value.variant() != IOTypeVariant::ImageFrame {
            return Err(IODataError::InvalidParameters("Initial Value for IdentityImageFrameCacheProcessor must be a ImageFrame!".into()).into());
        }
        Ok(Self { previous_value: initial_value })
    }
}