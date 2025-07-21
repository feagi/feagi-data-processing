use crate::error::{GenomeError, FeagiDataProcessingError, IODataError};

/// Represents 
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensionsRequirements {
    x: Option<u32>,
    y: Option<u32>,
    z: Option<u32>,
}

impl SingleChannelDimensionsRequirements {
    pub fn new(x: Option<u32>, y: Option<u32>, z: Option<u32>) -> Result<SingleChannelDimensionsRequirements, FeagiDataProcessingError> {
        if x.is_some_and(|x| x == 0) || y.is_some_and(|y| y == 0) || z.is_some_and(|z| z == 0) {
            return Err(GenomeError::InvalidChannelDimensions("Cortical Channel Dimensions cannot be 0 in any direction!".into()).into());
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
    
    pub fn is_ambiguous(&self) -> bool {
        !self.x.is_some() && self.y.is_some() && self.z.is_some()
    }
    
    pub fn describe_restrictions(&self) -> String {
        let mut output = String::new();
        if self.x.is_some() {
            output = format!("X axis length must be {},", self.x.unwrap());
        }
        else {
            output = format!("X axis length must be non-zero,");
        }
        
        if self.y.is_some() {
            output += format!("Y axis length must be {},", self.y.unwrap());
        }
        else {
            output = format!("Y axis length must be non-zero,");
        }
        
        if self.z.is_some() {
            output = format!("and Z axis length must be {}.", self.z.unwrap());
        }
        else {
            output = format!("and Z length must be non-zero.");
        }
        output
    }
}

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
    
    pub fn verify_restrictions(&self, verifying: &SingleChannelDimensionsRequirements) -> Result<(), FeagiDataProcessingError> {
        if verifying.x.is_some() {
            if verifying.x.unwrap() != self.x {
                return Err(IODataError::InvalidParameters((format!("Given X dimension value cannot be {}!", verifying.x.unwrap()) + &*verifying.describe_restrictions()).into()).into())
            }
        }

        if verifying.y.is_some() {
            if verifying.y.unwrap() != self.y {
                return Err(IODataError::InvalidParameters((format!("Given Y dimension value cannot be {}!", verifying.y.unwrap()) + &*verifying.describe_restrictions()).into()).into())
            }
        }

        if verifying.z.is_some() {
            if verifying.z.unwrap() != self.z {
                return Err(IODataError::InvalidParameters((format!("Given Z dimension value cannot be {}!", verifying.z.unwrap()) + &*verifying.describe_restrictions()).into()).into())
            }
        }
        
        Ok(())
    }
    
    
    
}
