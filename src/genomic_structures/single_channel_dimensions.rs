use crate::error::{GenomeError, FeagiDataProcessingError, IODataError};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensions {
    x: u32,
    y: u32,
    z: u32,
}

impl SingleChannelDimensions {
    pub fn new(x: u32, y: u32, z: u32) -> Result<SingleChannelDimensions, FeagiDataProcessingError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(GenomeError::InvalidChannelDimensions("Cortical Channel Dimensions cannot be 0 in any direction!".into()).into());
        }
        Ok(SingleChannelDimensions { x, y, z })
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }
    pub fn get_y(&self) -> u32 {
        self.y
    }
    pub fn get_z(&self) -> u32 {
        self.z
    }
    
    
    
}
