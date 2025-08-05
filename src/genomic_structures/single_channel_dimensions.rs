//! Single channel spatial dimensions for cortical areas.
//!
//! This module defines the `SingleChannelDimensions` type, which represents
//! the 3D spatial dimensions (X, Y, Z) of an individual channel within a
//! cortical area. These dimensions define the spatial structure and capacity
//! of neural processing within each channel.

use crate::error::{GenomeError, FeagiDataProcessingError};

/// Represents the 3D spatial dimensions of a single channel within a cortical area.
///
/// Each cortical area can contain multiple channels, and each channel has its own
/// 3D spatial structure defined by X, Y, and Z dimensions. These dimensions
/// determine the spatial organization and capacity of neural elements within
/// the channel.
///
/// # Coordinate System
/// - **X**: Width dimension (left-right)
/// - **Y**: Height dimension (up-down)  
/// - **Z**: Depth dimension (front-back)
///
/// # Constraints
/// All dimensions must be greater than 0. Zero-sized dimensions are not permitted
/// as they would represent invalid spatial structures.
///
/// # Usage
/// ```rust
/// use feagi_core_data_structures_and_processing::genomic_structures::SingleChannelDimensions;
///
/// // Create a 10x8x5 channel
/// let dimensions = SingleChannelDimensions::new(10, 8, 5).unwrap();
/// assert_eq!(dimensions.get_x(), 10);
/// assert_eq!(dimensions.get_y(), 8);
/// assert_eq!(dimensions.get_z(), 5);
/// ```
///
/// # Relationship to Cortical Areas
/// - Vision channels might be 640x480x1 for a camera resolution
/// - Audio channels might be 128x1x1 for frequency analysis
/// - Motor channels might be 1x1x10 for multi-axis control
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensions {
    x: u32,
    y: u32,
    z: u32,
}

impl SingleChannelDimensions {
    /// Creates new single channel dimensions with validation.
    ///
    /// All dimensions must be positive (greater than 0) to represent a valid
    /// spatial structure. Zero dimensions are rejected as they would create
    /// invalid cortical channel configurations.
    ///
    /// # Arguments
    /// * `x` - Width dimension (must be > 0)
    /// * `y` - Height dimension (must be > 0)
    /// * `z` - Depth dimension (must be > 0)
    ///
    /// # Returns
    /// * `Ok(SingleChannelDimensions)` - Valid dimensions object
    /// * `Err(FeagiDataProcessingError)` - If any dimension is 0
    ///
    /// # Example
    /// ```rust
    /// // Valid dimensions
    /// use feagi_core_data_structures_and_processing::genomic_structures::SingleChannelDimensions;
    /// let dims = SingleChannelDimensions::new(640, 480, 1).unwrap();
    ///
    /// // Invalid - will return error
    /// let invalid = SingleChannelDimensions::new(0, 480, 1);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(x: u32, y: u32, z: u32) -> Result<SingleChannelDimensions, FeagiDataProcessingError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(GenomeError::InvalidChannelDimensions("Cortical Channel Dimensions cannot be 0 in any direction!".into()).into());
        }
        Ok(SingleChannelDimensions { x, y, z })
    }

    /// Returns the X (width) dimension.
    ///
    /// # Returns
    /// The width dimension as a positive u32 value
    pub fn get_x(&self) -> u32 {
        self.x
    }

    /// Returns the Y (height) dimension.
    ///
    /// # Returns  
    /// The height dimension as a positive u32 value
    pub fn get_y(&self) -> u32 {
        self.y
    }

    /// Returns the Z (depth) dimension.
    ///
    /// # Returns
    /// The depth dimension as a positive u32 value
    pub fn get_z(&self) -> u32 {
        self.z
    }
    
    
    
}
