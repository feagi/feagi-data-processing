use std::fmt;
use crate::error::FeagiDataProcessingError;

// Since Implementations of this trait may differ in size or be dynamically sized, we cannot 
// turn this into an enum. Please Don't Try!

pub trait StreamCacheProcessor<T: fmt::Display> : fmt::Display {
    fn get_most_recent_output(&self) -> &T;
    
    fn process_new_input(&mut self, value: &T) -> Result<&T, FeagiDataProcessingError>; // Process in new input, get output
}