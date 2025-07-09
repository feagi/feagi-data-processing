use std::fmt;
use crate::error::FeagiDataProcessingError;
use crate::io_data::IOTypeData;

// Since Implementations of this trait may differ in size or be dynamically sized, we cannot 
// turn this into an enum. Please Don't Try!

pub trait StreamCacheProcessor: fmt::Display {
    fn get_most_recent_output(&self) -> &IOTypeData;
    
    fn process_new_input(&mut self, value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError>; // Process in new input, get output
}