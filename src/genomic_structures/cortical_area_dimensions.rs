use crate::error::{GenomeError, FeagiDataProcessingError};

/// Represents the dimensions of a cortical area. All dimension directions must be non-zero
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CorticalAreaDimensions {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl CorticalAreaDimensions {
    pub fn new(x: u32, y: u32, z: u32) -> Result<CorticalAreaDimensions, FeagiDataProcessingError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(GenomeError::InvalidCorticalDimensions("Cortical dimensions cannot be 0 in any direction!".into()).into());
        }
        Ok(CorticalAreaDimensions { x, y, z })
    }

    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (self.x, self.y, self.z)
    }

}