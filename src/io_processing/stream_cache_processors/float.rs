use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use super::StreamCacheProcessor;

#[derive(Debug, Clone)]
pub struct IdentityLinearFloatCacheProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityLinearFloatCacheProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity Linear Float processor value: {:?}", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityLinearFloatCacheProcessor {

    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::NormalizedM1to1F32
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::NormalizedM1to1F32
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        if value.variant() != self.get_input_data_type() {
            return Err(IODataError::InvalidParameters("Value for IdentityLinearFloatCacheProcessor must be a LinearNormalizedFloat!".into()).into());
        }
        self.previous_value = value;
        Ok(&self.previous_value) // No Change!
    }
}

impl IdentityLinearFloatCacheProcessor {
    pub fn new(initial_value: IOTypeData) -> Result<Self, FeagiDataProcessingError> {
        if initial_value.variant() != IOTypeVariant::NormalizedM1to1F32 {
            return Err(IODataError::InvalidParameters("Initial Value for IdentityLinearFloatCacheProcessor must be a LinearNormalizedFloat!".into()).into());
        }
        Ok(Self { previous_value: initial_value})
    }
}