use crate::error::GenomeError;

/// Represents the dimensions of a cortical area. All dimension directions must be non-zero
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CorticalDimensions {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl CorticalDimensions {
    pub fn new(x: u32, y: u32, z: u32) -> Result<CorticalDimensions, GenomeError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(GenomeError::InvalidCorticalDimensions("Cortical dimensions cannot be 0 in any direction!".into()));
        }
        Ok(CorticalDimensions { x, y, z })
    }

    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (self.x, self.y, self.z)
    }

    pub fn verify(&self) -> Result<(), GenomeError> {
        if self.x == 0 || self.y == 0 || self.z == 0 {
            return Err(GenomeError::InvalidCorticalDimensions("Cortical dimensions cannot be 0 in any direction!".into()));
        }
        Ok(())
    }

}