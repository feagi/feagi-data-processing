use crate::error::{GenomeError, FeagiDataProcessingError};

/// Represents the three-dimensional size of a cortical area in FEAGI
/// 
/// # Examples
/// 
/// ```
/// use feagi_core_data_structures_and_processing::genomic_structures::CorticalAreaDimensions;
/// 
/// // Create a small cortical area
/// let dimensions = CorticalAreaDimensions::new(10, 10, 5).unwrap();
/// assert_eq!(dimensions.as_tuple(), (10, 10, 5));
/// 
/// // This will fail because one dimension is zero
/// assert!(CorticalAreaDimensions::new(0, 10, 5).is_err());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CorticalAreaDimensions {
    /// The width of the cortical area (number of neurons along the x-axis)
    pub(crate) x: u32,
    /// The height of the cortical area (number of neurons along the y-axis)
    pub(crate) y: u32,
    /// The depth of the cortical area (number of neurons along the z-axis)
    pub(crate) z: u32,
}

impl CorticalAreaDimensions {
    /// Creates a new `CorticalAreaDimensions` instance with the specified dimensions.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The width dimension (number of neurons along x-axis). Must be > 0.
    /// * `y` - The height dimension (number of neurons along y-axis). Must be > 0.
    /// * `z` - The depth dimension (number of neurons along z-axis). Must be > 0.
    /// 
    /// # Returns
    /// 
    /// * `Ok(CorticalAreaDimensions)` - A valid cortical area dimensions instance
    /// * `Err(FeagiDataProcessingError)` - If any dimension is zero
    /// 
    /// # Errors
    /// 
    /// Returns a `GenomeError::InvalidCorticalDimensions` wrapped in a 
    /// `FeagiDataProcessingError` if any of the provided dimensions (x, y, or z) is zero.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalAreaDimensions;
    /// 
    /// // Valid dimensions
    /// let valid = CorticalAreaDimensions::new(50, 30, 20);
    /// assert!(valid.is_ok());
    /// 
    /// // Invalid dimensions (zero z-axis)
    /// let invalid = CorticalAreaDimensions::new(50, 30, 0);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(x: u32, y: u32, z: u32) -> Result<CorticalAreaDimensions, FeagiDataProcessingError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(GenomeError::InvalidCorticalDimensions("Cortical dimensions cannot be 0 in any direction!".into()).into());
        }
        Ok(CorticalAreaDimensions { x, y, z })
    }

    /// Returns the dimensions as a tuple in (x, y, z) order.
    /// 
    /// # Returns
    /// 
    /// A tuple `(u32, u32, u32)` containing the x, y, and z dimensions respectively.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalAreaDimensions;
    /// 
    /// let dimensions = CorticalAreaDimensions::new(100, 75, 25).unwrap();
    /// let (x, y, z) = dimensions.as_tuple();
    /// 
    /// assert_eq!(x, 100);
    /// assert_eq!(y, 75);
    /// assert_eq!(z, 25);
    /// ```
    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (self.x, self.y, self.z)
    }

}