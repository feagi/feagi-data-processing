//! Error handling for FEAGI data processing operations.
//!
//! This module defines the centralized error type [`DataProcessingError`] used throughout 
//! the FEAGI data processing library. It provides comprehensive error handling for various
//! failure modes that can occur during data processing operations.
//! ```

use std::fmt;

/// Comprehensive error type for all FEAGI data processing operations.
///
/// This enum covers all possible error conditions that can arise during data processing
/// in the FEAGI system, providing detailed error messages and context for debugging.
#[derive(Debug)]
pub enum DataProcessingError {
    /// Invalid input bounds or dimensions provided to a function.
    /// 
    /// This error occurs when input parameters are outside acceptable ranges,
    /// such as zero-length arrays where positive length is required, or
    /// coordinates that exceed defined boundaries.
    InvalidInputBounds(String),
    
    /// An in-place operation cannot be performed, typically due to incorrect memory dimensions
    /// 
    /// This error is returned when attempting to perform operations that modify
    /// data in-place but the target data structure doesn't support the operation
    /// or has incompatible dimensions.
    IncompatibleInplace(String),
    
    /// Input array format is incompatible with the expected structure for image processing.
    /// 
    /// This error occurs specifically in image processing contexts when the input
    /// array dimensions, channels, or data type don't match what's required for
    /// the operation.
    IncompatibleInputArray(String),
    
    /// An unexpected internal error occurred that indicates a bug in the library.
    /// 
    /// These errors represent programming errors or unexpected internal states
    /// that should be reported as issues. They typically indicate violations
    /// of internal invariants.
    InternalError(String),
    
    /// Required context or configuration is missing for the operation.
    /// 
    /// This error occurs when an operation requires additional context, configuration,
    /// or initialization that hasn't been provided.
    MissingContext(String),
    
    /// Invalid byte structure encountered during serialization or deserialization.
    /// 
    /// This error is returned when byte data doesn't conform to the expected
    /// structure format, such as incorrect headers, invalid lengths, or
    /// malformed content.
    InvalidByteStructure(String),
    
    /// The requested functionality is not yet implemented.
    /// 
    /// This error indicates that a feature is planned but hasn't been completed yet.
    /// Users encountering this error should check for updates or file an issue
    /// requesting the feature.
    NotImplemented,
}

impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            DataProcessingError::InvalidInputBounds(e) => write!(f, "Invalid Bounds! {}", e),
            DataProcessingError::IncompatibleInplace(e) => write!(f, "Unable to perform inplace operation! {}", e),
            DataProcessingError::IncompatibleInputArray(e) => write!(f, "Incompatible input array for images! {}", e),
            DataProcessingError::InternalError(e) => write!(f, "Internal error! Please report this! {}", e),
            DataProcessingError::MissingContext(e) => write!(f, "Missing context! {}", e),
            DataProcessingError::InvalidByteStructure(e) => write!(f, "Invalid byte Structure! {}", e),
            DataProcessingError::NotImplemented => write!(f, "This function is not yet implemented! Please raise an issue on Github!"),
        }
    }
}