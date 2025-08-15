//! Range scaling processors for normalizing float values to specific ranges.
//!
//! This module provides processors that linearly scale input float values from a specified 
//! input range to normalized output ranges. These processors are commonly used to normalize
//! sensor data or other continuous values for FEAGI processing.

use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::StreamCacheProcessor;

/// A stream processor that linearly scales input float values to the range [0, 1].
///
/// This processor takes float values within a specified input range [lower_bound, upper_bound]
/// and maps them linearly to the normalized range [0, 1]. Values outside the given bounds
/// are clamped to the bounds before scaling.
///
/// # Example
/// ```
/// // Scale values from range [10, 50] to [0, 1]
/// use feagi_core_data_structures_and_processing::io_processing::processors::LinearScaleTo0And1Processor;
/// let mut processor = LinearScaleTo0And1Processor::new(10.0, 50.0, 30.0).unwrap();
/// // Input 30.0 would map to 0.5 in the output range
/// ```
#[derive(Debug, Clone)]
pub struct LinearScaleTo0And1Processor {
    previous_value: IOTypeData,
    lower: f32,
    upper: f32,
    upper_minus_lower: f32
}

impl Display for LinearScaleTo0And1Processor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinearScaleTo0And1(lower_bound={:?},upper_bound={:?},prev_val={:?})", self.lower, self.upper,  self.previous_value)
    }
}

impl StreamCacheProcessor for LinearScaleTo0And1Processor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32Normalized0To1
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData, _: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let float_result = f32::try_from(value)?;
        let clamped = float_result.clamp(self.lower, self.upper);
        let val_0_1 = (clamped - self.lower) / self.upper_minus_lower;

        self.previous_value = IOTypeData::F32Normalized0To1(val_0_1);
        Ok(&self.previous_value)
    }
}

impl LinearScaleTo0And1Processor {
    /// Creates a new LinearScaleTo0And1 processor.
    ///
    /// # Arguments
    /// * `lower_bound` - The minimum value of the input range
    /// * `upper_bound` - The maximum value of the input range (must be > lower_bound)
    /// * `initial_value` - The initial value to store in the processor (must be within bounds)
    ///
    /// # Returns
    /// * `Ok(LinearScaleTo0And1)` - A new processor instance
    /// * `Err(FeagiDataProcessingError)` - If parameters are invalid (NaN, infinite, or out of bounds)
    pub fn new(lower_bound: f32, upper_bound: f32, initial_value: f32) -> Result<Self, FeagiDataProcessingError> {
        if lower_bound.is_nan() || lower_bound.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given lower bound float {} is not valid!", lower_bound)).into());
        }
        if upper_bound.is_nan() || upper_bound.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given upper bound float {} is not valid!", upper_bound)).into());
        }
        if initial_value.is_nan() || initial_value.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given initial value float {} is not valid!", initial_value)).into());
        }
        if upper_bound < lower_bound {
            return Err(IODataError::InvalidParameters(format!("Upper bound float {} must be greater than lower bound {}!!", upper_bound, lower_bound)).into());
        }
        if initial_value > upper_bound || initial_value < lower_bound {
            return Err(IODataError::InvalidParameters(format!("Initial value float {} must be between bounds {} and {}!", initial_value, lower_bound, upper_bound)).into());
        }

        Ok(LinearScaleTo0And1Processor {
            previous_value: IOTypeData::F32Normalized0To1(initial_value),
            lower: lower_bound,
            upper: upper_bound,
            upper_minus_lower: upper_bound - lower_bound,
        })
    }
}

/// A stream processor that linearly scales input float values to the range [-1, 1].
///
/// This processor takes float values within a specified input range [lower_bound, upper_bound]
/// and maps them linearly to the normalized range [-1, 1]. Values outside the given bounds
/// are clamped to the bounds before scaling.
///
/// # Example
/// ```
/// // Scale values from range [0, 100] to [-1, 1]
/// use feagi_core_data_structures_and_processing::io_processing::processors::LinearScaleToM1And1;
/// let mut processor = LinearScaleToM1And1::new(0.0, 100.0, 50.0).unwrap();
/// // Input 50.0 would map to 0.0 in the output range
/// ```
#[derive(Debug, Clone)]
pub struct LinearScaleToM1And1 {
    previous_value: IOTypeData,
    lower: f32,
    upper: f32,
    upper_minus_lower_halved: f32
}

impl Display for LinearScaleToM1And1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinearScaleToM1And1(lower_bound={:?},upper_bound={:?},prev_val={:?})", self.lower, self.upper,  self.previous_value)
    }
}

impl StreamCacheProcessor for LinearScaleToM1And1 {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32NormalizedM1To1
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData, _: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let float_result = f32::try_from(value)?;
        let clamped = float_result.clamp(self.lower, self.upper);
        let val_m1_1 = ((clamped - self.lower) / self.upper_minus_lower_halved) - 1.0;

        self.previous_value = IOTypeData::F32NormalizedM1To1(val_m1_1);
        Ok(&self.previous_value)
    }
}

impl LinearScaleToM1And1 {
    /// Creates a new LinearScaleToM1And1 processor.
    ///
    /// # Arguments
    /// * `lower_bound` - The minimum value of the input range
    /// * `upper_bound` - The maximum value of the input range (must be > lower_bound)
    /// * `initial_value` - The initial value to store in the processor (must be within bounds)
    ///
    /// # Returns
    /// * `Ok(LinearScaleToM1And1)` - A new processor instance
    /// * `Err(FeagiDataProcessingError)` - If parameters are invalid (NaN, infinite, or out of bounds)
    pub fn new(lower_bound: f32, upper_bound: f32, initial_value: f32) -> Result<Self, FeagiDataProcessingError> {
        if lower_bound.is_nan() || lower_bound.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given lower bound float {} is not valid!", lower_bound)).into());
        }
        if upper_bound.is_nan() || upper_bound.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given upper bound float {} is not valid!", upper_bound)).into());
        }
        if initial_value.is_nan() || initial_value.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given initial value float {} is not valid!", initial_value)).into());
        }
        if upper_bound < lower_bound {
            return Err(IODataError::InvalidParameters(format!("Upper bound float {} must be greater than lower bound {}!!", upper_bound, lower_bound)).into());
        }
        if initial_value > upper_bound || initial_value < lower_bound {
            return Err(IODataError::InvalidParameters(format!("Initial value float {} must be between bounds {} and {}!", initial_value, lower_bound, upper_bound)).into());
        }

        Ok(LinearScaleToM1And1 {
            previous_value: IOTypeData::F32NormalizedM1To1(initial_value),
            lower: lower_bound,
            upper: upper_bound,
            upper_minus_lower_halved: (upper_bound - lower_bound) * 0.5,
        })
    }
}