use std::ops::Range;
use crate::error::{FeagiDataProcessingError};

/// Defines the acceptable range of values for each dimensions
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ChannelDimensionRange { // Must be exposed due to usage in down stream wrappers
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

impl ChannelDimensionRange {
    pub fn new(x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Result<ChannelDimensionRange, FeagiDataProcessingError> {
        if x.is_empty() || y.is_empty() || z.is_empty() {
            return Err(FeagiDataProcessingError::InternalError("A given range appears invalid!".into()))
        }
        
        if x.len() == 0 || y.len() == 0 || z.len() == 0 {
            return Err(FeagiDataProcessingError::InternalError("A given range appears empty!".into()))
        }
        Ok(ChannelDimensionRange { x, y, z })
    }

    pub fn is_ambiguous(&self) -> bool {
        self.x.len() == 1 && self.y.len() == 1 && self.z.len() == 1
    }
    
    // TODO other methods
}