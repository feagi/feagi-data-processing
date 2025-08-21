//! Core trait definition for stream cache processors.
//!
//! This module defines the `StreamCacheProcessor` trait, which provides a unified interface
//! for all data transformation processors in the FEAGI system. Processors implementing this
//! trait can be chained together to create complex data processing pipelines.

use std::fmt;
use std::fmt::Debug;
use std::time::Instant;
use crate::error::FeagiDataProcessingError;
use crate::io_data::{IOTypeData, IOTypeVariant};

/// Core trait for stream cache processors that transform data in real-time pipelines.
///
/// This trait defines the interface that all stream processors must implement to participate
/// in FEAGI's data processing pipelines. Processors are stateful basic_components that can transform,
/// filter, or aggregate incoming data streams.
///
/// # Design Principles
///
/// - **Type Safety**: Each processor declares its input and output data types
/// - **Statefulness**: Processors maintain internal state and can reference previous values
/// - **Thread Safety**: All processors must be `Sync + Send` for concurrent processing
/// - **Chainable**: Processors can be linked together when output types match input types
///
/// # Implementation Requirements
///
/// Implementors must provide:
/// - Type declarations for input and output data
/// - Stateful processing logic that handles new input values
/// - Access to the most recently processed output
/// - Display formatting for debugging and logging
///
/// # Note on Design
/// Since implementations may differ in size or be dynamically sized, this cannot
/// be converted to an enum. Each processor type maintains its own internal state structure.
///
/// # Example
///
/// ```rust
/// use std::fmt::{Display, Formatter};
/// use feagi_core_data_structures_and_processing::{
///     io_processing::StreamCacheProcessor,
///     io_data::{IOTypeData, IOTypeVariant},
///     error::FeagiDataProcessingError
/// };
/// use std::time::Instant;
///
/// #[derive(Debug, Clone)]
/// struct MyProcessor {
///     last_value: IOTypeData,
/// }
///
/// impl StreamCacheProcessor for MyProcessor {
///     fn get_input_data_type(&self) -> IOTypeVariant {
///         IOTypeVariant::F32
///     }
///
///     fn get_output_data_type(&self) -> IOTypeVariant {
///         IOTypeVariant::F32
///     }
///
///     fn get_most_recent_output(&self) -> &IOTypeData {
///         &self.last_value
///     }
///
///     fn process_new_input(&mut self, value: &IOTypeData, _time: Instant) 
///         -> Result<&IOTypeData, FeagiDataProcessingError> {
///         // Transform the input somehow
///         self.last_value = value.clone();
///         Ok(&self.last_value)
///     }
/// }
///
/// impl Display for MyProcessor {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "MyProcessor({:?})", self.last_value)
///     }
/// }
/// ```
pub trait StreamCacheProcessor: fmt::Display + Debug + Sync + Send {
    /// Returns the data type this processor expects as input.
    ///
    /// This is used by `ProcessorRunner` to validate that processors can be chained
    /// together correctly (output type of one matches input type of the next).
    fn get_input_data_type(&self) -> IOTypeVariant;

    /// Returns the data type this processor produces as output.
    ///
    /// This is used by `ProcessorRunner` to validate processor chain compatibility
    /// and determine the final output type of processing pipeline.
    fn get_output_data_type(&self) -> IOTypeVariant;
    
    /// Returns a reference to the most recently processed output value from internal cached memory.
    fn get_most_recent_output(&self) -> &IOTypeData;

    /// Process new input data and return a reference to the transformed output.
    ///
    /// This is the core processing method where the actual data transformation occurs.
    /// Implementations should:
    /// - Transform the input value according to their specific logic
    /// - Update internal state with the new output
    /// - Return a reference to the newly computed output
    ///
    /// # Arguments
    /// * `value` - The input data to process (type should match `get_input_data_type()`)
    /// * `time_of_input` - Timestamp when the input was received (for time-aware processing)
    ///
    /// # Returns
    /// * `Ok(&IOTypeData)` - Reference to the processed output data
    /// * `Err(FeagiDataProcessingError)` - If processing fails due to invalid input or internal errors
    ///
    /// # Note
    /// Type checking is not performed here - it's the responsibility of `ProcessorRunner`
    /// to ensure input types are compatible before calling this method.
    fn process_new_input(&mut self, value: &IOTypeData, time_of_input: Instant) -> Result<&IOTypeData, FeagiDataProcessingError>;
    
}

// TODO JSON descriptors and parameter updates