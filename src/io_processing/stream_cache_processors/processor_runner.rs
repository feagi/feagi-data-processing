//! Processor runner for orchestrating chains of stream cache processors.
//!
//! This module provides the `ProcessorRunner` struct, which validates and executes
//! chains of `StreamCacheProcessor` instances. It ensures type compatibility between
//! processors and manages the flow of data through the processing pipeline.

use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::StreamCacheProcessor;

/// Orchestrates execution of a chain of stream cache processors.
///
/// `ProcessorRunner` validates that a sequence of processors can be chained together
/// (ensuring output types match subsequent input types) and provides a unified interface
/// for processing data through the entire pipeline.
///
/// # Key Features
///
/// - **Type Validation**: Ensures processors are compatible before execution
/// - **Pipeline Execution**: Runs data through all processors in sequence
/// - **Error Handling**: Provides clear error messages for incompatible processors
/// - **Performance**: Uses efficient borrowing patterns to avoid unnecessary clones
#[derive(Debug)]
pub(crate) struct ProcessorRunner {
    input_type: IOTypeVariant,
    output_type: IOTypeVariant,
    cache_processors: Vec<Box<dyn StreamCacheProcessor + Sync + Send>>,
}

impl ProcessorRunner {
    /// Creates a new ProcessorRunner with a validated chain of processors.
    ///
    /// This constructor performs comprehensive validation to ensure the processor chain
    /// is valid and can execute successfully:
    /// - Checks that at least one processor is provided
    /// - Validates type compatibility between adjacent processors
    /// - Determines the overall input and output types for the pipeline
    ///
    /// # Arguments
    /// * `cache_processors` - Vector of processors to chain together (must be non-empty)
    ///
    /// # Returns
    /// * `Ok(ProcessorRunner)` - A validated processor runner ready for execution
    /// * `Err(FeagiDataProcessingError)` - If validation fails:
    ///   - Empty processor list
    ///   - Type incompatibility between adjacent processors
    ///
    /// # Type Compatibility Rules
    /// For processors to be compatible in a chain, each processor's output type
    /// must exactly match the next processor's input type:
    /// ```text
    /// Processor A: Input(F32) -> Output(F32Normalized0To1)
    /// Processor B: Input(F32Normalized0To1) -> Output(Bool)  ✓ Compatible
    /// 
    /// Processor A: Input(F32) -> Output(F32Normalized0To1)
    /// Processor B: Input(F32) -> Output(Bool)              ✗ Incompatible
    /// ```
    pub fn new(cache_processors: Vec<Box<dyn StreamCacheProcessor + Sync + Send>>) -> Result<Self, FeagiDataProcessingError> {
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
    
    /// Processes new input data through the entire processor chain.
    ///
    /// Takes input data, validates it matches the expected input type, then runs it
    /// sequentially through all processors in the chain. Each processor's output
    /// becomes the input for the next processor.
    ///
    /// # Arguments
    /// * `new_value` - Input data to process (must match the chain's input type)
    /// * `time_of_update` - Timestamp for when this update occurred
    ///
    /// # Returns
    /// * `Ok(&IOTypeData)` - Reference to the final processed output from the last processor
    /// * `Err(FeagiDataProcessingError)` - If processing fails:
    ///   - Input type doesn't match expected type
    ///   - Any processor in the chain fails to process its input
    ///
    /// # Processing Flow
    /// 1. Validate input type matches the chain's expected input type
    /// 2. Process input through first processor
    /// 3. For each subsequent processor, use previous processor's output as input
    /// 4. Return final output from the last processor
    ///
    /// # Performance Notes
    /// Uses `split_at_mut` to avoid borrowing conflicts when accessing processor outputs
    /// while mutating subsequent processors in the chain.
    pub fn update_value(&mut self, new_value: &IOTypeData, time_of_update: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        if IOTypeVariant::from(new_value) != self.input_type {
            return Err(IODataError::InvalidParameters(format!("Expected Input data type of {} but received {}!", self.input_type.to_string(), new_value.to_string())).into());
        }

        //TODO There has to be a better way to do this, but I keep running into limitations with mutating self.cache_processors
        
        // Process the first processor with the input value
        self.cache_processors[0].process_new_input(new_value, time_of_update)?;
        
        // Process subsequent processors using split_at_mut to avoid borrowing conflicts
        for i in 1..self.cache_processors.len() {
            let (left, right) = self.cache_processors.split_at_mut(i);
            let previous_output = left[i - 1].get_most_recent_output(); 
            right[0].process_new_input(previous_output, time_of_update)?;
        }
        
        // Return the output from the last processor
        Ok(self.cache_processors.last().unwrap().get_most_recent_output())
    }
    
    /// Returns the most recent output from the final processor in the chain.
    ///
    /// This provides access to the current state of the processing pipeline without
    /// triggering new processing. Useful for reading the current processed value.
    ///
    /// # Returns
    /// Reference to the output data from the last processor in the chain.
    pub fn get_most_recent_output(&self) -> &IOTypeData {
        self.cache_processors.last().unwrap().get_most_recent_output()
    }
    
    /// Returns the input data type expected by this processor chain.
    ///
    /// This is determined by the input type of the first processor in the chain.
    /// Used for validation before processing new input data.
    pub fn get_input_data_type(&self) -> IOTypeVariant {
        self.input_type
    }
    
    /// Returns the output data type produced by this processor chain.
    ///
    /// This is determined by the output type of the last processor in the chain.
    /// Useful for understanding what type of data the pipeline will produce.
    pub fn get_output_data_type(&self) -> IOTypeVariant {
        self.output_type
    }
}