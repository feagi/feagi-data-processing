use crate::error::GenomeError;

/// Represents 
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensionsRequirements {
    x: Option<u32>,
    y: Option<u32>,
    z: Option<u32>,
}

impl SingleChannelDimensionsRequirements {
    pub fn new(x: Option<u32>, y: Option<u32>, z: Option<u32>) -> Result<SingleChannelDimensionsRequirements, GenomeError> {
        if x.is_some_and(|x| x == 0) || y.is_some_and(|y| y == 0) || z.is_some_and(|z| z == 0) {
            return Err(GenomeError::InvalidChannelDimensions("Cortical Channel Dimensions cannot be 0 in any direction!".into()));
        }
        Ok(SingleChannelDimensionsRequirements { x, y, z })
    }
    
    pub fn get_x(&self) -> &Option<u32> {
        &self.x
    }
    pub fn get_y(&self) -> &Option<u32> {
        &self.y
    }
    pub fn get_z(&self) -> &Option<u32> {
        &self.z
    }
    
    pub fn is_ambigious(&self) -> bool {
        !self.x.is_some() && self.y.is_some() && self.z.is_some()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensions {
    x: u32,
    y: u32,
    z: u32,
}

impl SingleChannelDimensions {
    pub fn new(x: u32, y: u32, z: u32) -> Result<SingleChannelDimensions, GenomeError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(GenomeError::InvalidChannelDimensions("Cortical Channel Dimensions cannot be 0 in any direction!".into()));
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
