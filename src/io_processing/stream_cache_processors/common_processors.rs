use std::fmt::{Display, Formatter};
use crate::error::FeagiDataProcessingError;
use super::super::StreamCacheProcessor;

pub struct IdentityCacheProcessor<T: Display + Clone> {
    previous_value: T,
}

impl<T: Display + Clone> Display for IdentityCacheProcessor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity processor value: {}", self.previous_value)
    }
}

impl<T: Display + Clone> StreamCacheProcessor<T> for IdentityCacheProcessor<T> {
    fn get_most_recent_output(&self) -> &T {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &T) -> Result<&T, FeagiDataProcessingError> {
        self.previous_value = value.clone();
        Ok(&self.previous_value)
    }
}
