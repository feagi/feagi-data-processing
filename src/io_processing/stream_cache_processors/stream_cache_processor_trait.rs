use std::fmt;
use std::fmt::Debug;
use std::time::Instant;
use crate::error::FeagiDataProcessingError;
use crate::io_data::{IOTypeData, IOTypeVariant};

// Since Implementations of this trait may differ in size or be dynamically sized, we cannot 
// turn this into an enum. Please Don't Try!

pub trait StreamCacheProcessor: fmt::Display + Debug + Sync {
    fn get_input_data_type(&self) -> IOTypeVariant;

    fn get_output_data_type(&self) -> IOTypeVariant;
    
    fn get_most_recent_output(&self) -> &IOTypeData;
    
    fn process_new_input(&mut self, value: IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError>; // Process in new input, get output
    // NOTE: We will not do any explicit checks here as we already verify type safety explicitly on instantiation
    
}