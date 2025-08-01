//! Rolling window processors for temporal smoothing of data streams.
//!
//! This module provides processors that maintain a sliding window of recent values
//! and compute statistics (like averages) over that window. These processors are
//! useful for smoothing noisy sensor data or computing temporal aggregations.

use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::StreamCacheProcessor;

/// A stream processor that maintains a rolling window of float values and outputs their average.
///
/// This processor keeps track of the last N values (where N is the window length) and
/// continuously outputs the arithmetic mean of these values. When a new value arrives,
/// it replaces the oldest value in the window using a circular buffer approach.
///
/// # Example
/// ```
/// // Create a rolling average with window size of 5
/// use feagi_core_data_structures_and_processing::io_processing::processors::LinearAverageRollingWindowProcessor;
/// let mut processor = LinearAverageRollingWindowProcessor::new(5, 0.0).unwrap();
/// // Each new input will be averaged with the previous 4 values
/// ```
#[derive(Debug, Clone)]
pub struct LinearAverageRollingWindowProcessor {
    previous_value: IOTypeData,
    last_index: usize,
    window: Vec<f32>,
    window_length: f32 // While this is treated an int, we do divisions so we keep this as a f32
}

impl Display for LinearAverageRollingWindowProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinearAverageRollingWindowProcessor(length={:?},prev_val={:?})", self.window.len(),  self.previous_value)
    }
}

impl StreamCacheProcessor for LinearAverageRollingWindowProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.previous_value
    }

    fn process_new_input(&mut self, value: &IOTypeData, _: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let float_result = f32::try_from(value)?;
        let new_index = (self.last_index + 1) % self.window.len();
        self.window[new_index] = float_result;

        self.last_index += 1;

        self.previous_value = IOTypeData::F32(self.window.iter().sum::<f32>() / self.window_length); // average
        Ok(&self.previous_value)
    }
}

impl LinearAverageRollingWindowProcessor {
    /// Creates a new LinearAverageRollingWindowProcessor.
    ///
    /// # Arguments
    /// * `window_length` - The size of the rolling window (must be > 0)
    /// * `initial_value` - The initial value to fill the window with (must be finite)
    ///
    /// # Returns
    /// * `Ok(LinearAverageRollingWindowProcessor)` - A new processor instance with window filled with initial_value
    /// * `Err(FeagiDataProcessingError)` - If window_length is 0 or initial_value is invalid (NaN/infinite)
    pub fn new(window_length: usize, initial_value: f32) -> Result<Self, FeagiDataProcessingError> {
        if initial_value.is_nan() || initial_value.is_infinite() {
            return Err(IODataError::InvalidParameters(format!("Given float {} is not valid!", initial_value)).into());
        }
        if window_length == 0 {
            return Err(IODataError::InvalidParameters(format!("Window length cannot be 0!")).into());
        }

        Ok(LinearAverageRollingWindowProcessor{
            previous_value: IOTypeData::F32(initial_value),
            last_index: 0,
            window: vec![initial_value; window_length],
            window_length: window_length as f32
        })
    }
}