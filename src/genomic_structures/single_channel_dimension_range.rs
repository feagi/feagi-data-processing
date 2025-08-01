//! Validation ranges for single channel dimensions.
//!
//! This module provides the `SingleChannelDimensionRange` type, which defines
//! acceptable ranges for each spatial dimension of cortical area channels.
//! These ranges are used to validate that channel dimensions conform to the
//! constraints imposed by specific cortical area types.

use std::ops::Range;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::SingleChannelDimensions;

/// Defines acceptable ranges for each spatial dimension of a cortical channel.
///
/// Different cortical area types have different dimensional requirements and constraints.
/// This type encapsulates those constraints as ranges for each spatial dimension,
/// allowing validation of proposed channel dimensions against type-specific rules.
///
/// # Purpose
/// - **Validation**: Ensure channel dimensions are appropriate for cortical area types
/// - **Constraints**: Enforce limits and requirements
/// - **Type Safety**: Prevent invalid cortical area configurations
///
/// # Range Types
/// - **Fixed**: Range with length 1 (e.g., 1..2 means exactly 1)
/// - **Bounded**: Range with specific min/max limits (e.g., 1..100)
/// - **Open**: Range with very large upper bound (e.g., 1..u32::MAX)
///
/// # Examples by Cortical Type
/// - **Vision**: X: 1..4096, Y: 1..4096, Z: 1..10 (width×height×channels)
/// - **Motor**: X: 1..2, Y: 1..2, Z: 1..32 (simple×simple×axes)
/// - **Core**: X: 1..2, Y: 1..2, Z: 1..2 (minimal fixed size)
///
/// # Usage
/// ```rust
/// use feagi_core_data_structures_and_processing::genomic_structures::*;
///
/// // Define a range for vision-type channels
/// let vision_range = SingleChannelDimensionRange::new(
///     1..1920,  // Width: 1 to 1920 pixels
///     1..1080,  // Height: 1 to 1080 pixels  
///     1..4      // Depth: 1 to 3 color channels
/// ).unwrap();
///
/// // Validate proposed dimensions
/// let proposed = SingleChannelDimensions::new(640, 480, 3).unwrap();
/// vision_range.verify_within_range(&proposed).unwrap(); // Should pass
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensionRange { // Must be exposed due to usage in down stream wrappers
    x: Range<u32>,
    y: Range<u32>,
    z: Range<u32>,
}

impl SingleChannelDimensionRange {
    /// Creates a new dimension range with validation.
    ///
    /// All provided ranges must be non-empty and valid. Empty ranges would
    /// prevent any valid dimensions from being created, which is not useful
    /// for cortical area configuration.
    ///
    /// # Arguments
    /// * `x` - Range of acceptable X (width) dimension values
    /// * `y` - Range of acceptable Y (height) dimension values  
    /// * `z` - Range of acceptable Z (depth) dimension values
    ///
    /// # Returns
    /// * `Ok(SingleChannelDimensionRange)` - Valid dimension range
    /// * `Err(FeagiDataProcessingError)` - If any range is empty or invalid
    ///
    /// # Example
    /// ```rust
    /// // Create a range for small square images with minimal depth
    /// use feagi_core_data_structures_and_processing::genomic_structures::SingleChannelDimensionRange;
    /// let range = SingleChannelDimensionRange::new(
    ///     1..256,   // Width: 1 to 255 pixels
    ///     1..256,   // Height: 1 to 255 pixels
    ///     1..4      // Depth: 1 to 3 channels
    /// ).unwrap();
    /// ```
    pub fn new(x: Range<u32>, y: Range<u32>, z: Range<u32>) -> Result<SingleChannelDimensionRange, FeagiDataProcessingError> {
        if x.is_empty() || y.is_empty() || z.is_empty() {
            return Err(FeagiDataProcessingError::InternalError("A given range appears invalid!".into()))
        }
        
        if x.len() == 0 || y.len() == 0 || z.len() == 0 {
            return Err(FeagiDataProcessingError::InternalError("A given range appears empty!".into()))
        }
        Ok(SingleChannelDimensionRange { x, y, z })
    }

    /// Checks if this range specifies exact dimensions (no flexibility).
    ///
    /// Returns true if all three dimensional ranges have exactly one valid value,
    /// meaning the dimensions are completely fixed with no variation allowed.
    /// This is common for core cortical areas that have predetermined sizes.
    ///
    /// # Returns
    /// * `true` - All dimensions are fixed to exact values
    /// * `false` - At least one dimension allows multiple values
    ///
    /// # Example
    /// ```rust
    /// // Fixed dimensions (all ranges have length 1)
    /// use feagi_core_data_structures_and_processing::genomic_structures::SingleChannelDimensionRange;
    /// let fixed = SingleChannelDimensionRange::new(5..6, 10..11, 1..2).unwrap();
    /// assert!(fixed.is_ambiguous());
    ///
    /// // Flexible dimensions
    /// let flexible = SingleChannelDimensionRange::new(1..100, 1..100, 1..4).unwrap();
    /// assert!(!flexible.is_ambiguous());
    /// ```
    pub fn is_ambiguous(&self) -> bool {
        self.x.len() == 1 && self.y.len() == 1 && self.z.len() == 1
    }
    
    /// Validates that given dimensions fall within the acceptable ranges.
    ///
    /// Checks each dimension (X, Y, Z) of the provided channel dimensions against
    /// the corresponding acceptable range. All dimensions must be within their
    /// respective ranges for validation to pass.
    ///
    /// # Arguments
    /// * `checking` - The channel dimensions to validate
    ///
    /// # Returns
    /// * `Ok(())` - All dimensions are within acceptable ranges
    /// * `Err(FeagiDataProcessingError)` - One or more dimensions are out of range
    ///
    /// # Error Details
    /// The error message includes:
    /// - The actual dimension values that failed validation
    /// - Detailed range information for each axis showing what values are acceptable
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{SingleChannelDimensionRange, SingleChannelDimensions};
    /// let range = SingleChannelDimensionRange::new(1..100, 1..100, 1..5).unwrap();
    /// let valid_dims = SingleChannelDimensions::new(50, 75, 3).unwrap();
    /// let invalid_dims = SingleChannelDimensions::new(150, 75, 3).unwrap();
    ///
    /// assert!(range.verify_within_range(&valid_dims).is_ok());
    /// assert!(range.verify_within_range(&invalid_dims).is_err());
    /// ```
    pub fn verify_within_range(&self, checking: &SingleChannelDimensions) -> Result<(), FeagiDataProcessingError> {
        if self.x.contains(&checking.get_x()) && self.y.contains(&checking.get_y()) && self.z.contains(&checking.get_z()) {
            return Ok(())
        }
        Err(IODataError::InvalidParameters(format!("Given channel dimensions of ({:?},{:?}, {:?}) that do not fit in acceptable range! {}", 
                                                   checking.get_x(), checking.get_y(), checking.get_z(), self.print_axis_ranges()?)).into())
    }
    
    
    fn print_axis_ranges(&self) -> Result<String, FeagiDataProcessingError> {
        Ok(format!("{}. {}. {}.", self.print_axis_range(0)?, self.print_axis_range(1)?, self.print_axis_range(2)?))
    }
    
    fn print_axis_range(&self, axis_index: usize) -> Result<String, FeagiDataProcessingError> {
        let axis_label: &str;
        let axis = match axis_index {
            0 => {
                axis_label = "X";
                &self.x},
            1 => {
                axis_label = "Y";
                &self.y},
            2 => {
                axis_label = "Z";
                &self.z},
            _ => return Err(FeagiDataProcessingError::InternalError("Invalid axis_index!".into()))
        };
        
        if axis.len() == 1 {
            return Ok(format!("Axis {:?} must be {:?}", axis_label, axis.start));
        }
        Ok(format!("Axis {:?} must be equal or greater than {:?} and less than {:?}", axis_label, axis.start, axis.end))
    }
    
    
    
    // TODO other methods
}