use std::fmt::{Display, Formatter};
use crate::error::FeagiDataProcessingError;
use crate::io_data::IOTypeData;
use super::super::StreamCacheProcessor;

pub struct IdentityCacheProcessor {
    previous_value: IOTypeData,
}

impl Display for IdentityCacheProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity processor value: {:?}", self.previous_value)
    }
}

impl StreamCacheProcessor for IdentityCacheProcessor {
    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}
