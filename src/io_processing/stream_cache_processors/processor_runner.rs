use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::StreamCacheProcessor;

// Verifies a vector of StreamCacheProcessor is compatible, and runs through them when given data.

#[derive(Debug)]
pub(crate) struct ProcessorRunner {
    input_type: IOTypeVariant,
    output_type: IOTypeVariant,
    cache_processors: Vec<Box<dyn StreamCacheProcessor>>,
}

impl ProcessorRunner {
    pub fn new(cache_processors: Vec<Box<dyn StreamCacheProcessor>>) -> Result<Self, FeagiDataProcessingError> {
        let number_of_processors = cache_processors.len();
        
        if number_of_processors == 0 {
            return Err(IODataError::InvalidParameters("Processor Runner cannot have 0 Cache Processors!".into()).into())
        }
        
        if number_of_processors == 1 {
            let processor = &cache_processors[0];
            return Ok(ProcessorRunner {
                input_type: processor.get_input_data_type(),
                output_type: processor.get_output_data_type(),
                cache_processors,
            });
        }
        
        
        // Ensure data can pass between processors
        for processer_index in 0..number_of_processors - 1  {
            let first = &cache_processors[processer_index];
            let second = &cache_processors[processer_index + 1];
            if first.get_output_data_type() != second.get_input_data_type() {
                return Err(IODataError::InvalidParameters(format!("Given cache processor at index {} has output type {}, which does not match the input type of cache processor at index {} or type {}!",
                processer_index, first.get_output_data_type(), processer_index + 1,  second.get_input_data_type()).into()).into());
            }
        };
        
        Ok(ProcessorRunner {
            input_type: cache_processors.first().unwrap().get_input_data_type(),
            output_type: cache_processors.last().unwrap().get_output_data_type(),
            cache_processors,
        })
    }
    
    pub fn update_value(&mut self, new_value: &IOTypeData) -> Result<&IOTypeData, FeagiDataProcessingError> {
        if IOTypeVariant::from(new_value) != self.input_type {
            return Err(IODataError::InvalidParameters(format!("Expected Input data type of {} but received {}!", self.input_type.to_string(), new_value.to_string())).into());
        }

        //TODO There has to be a better way to do this, but I keep running into limitations with mutating self.cache_processors
        
        // Process the first processor with the input value
        self.cache_processors[0].process_new_input(new_value)?;
        
        // Process subsequent processors using split_at_mut to avoid borrowing conflicts
        for i in 1..self.cache_processors.len() {
            let (left, right) = self.cache_processors.split_at_mut(i);
            let previous_output = left[i - 1].get_most_recent_output(); 
            right[0].process_new_input(previous_output)?;
        }
        
        // Return the output from the last processor
        Ok(self.cache_processors.last().unwrap().get_most_recent_output())
    }
    
    pub fn get_most_recent_output(&self) -> &IOTypeData {
        self.cache_processors.last().unwrap().get_most_recent_output()
    }
    
    pub fn get_input_data_type(&self) -> IOTypeVariant {
        self.input_type
    }
    
    pub fn get_output_data_type(&self) -> IOTypeVariant {
        self.output_type
    }
}