use std::ops::Range;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::SingleChannelDimensions;

/// Defines the acceptable range of values for each dimensions
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SingleChannelDimensionRange { // Must be exposed due to usage in down stream wrappers
    x: Range<u32>,
    y: Range<u32>,
    z: Range<u32>,
}

impl SingleChannelDimensionRange {
    pub fn new(x: Range<u32>, y: Range<u32>, z: Range<u32>) -> Result<SingleChannelDimensionRange, FeagiDataProcessingError> {
        if x.is_empty() || y.is_empty() || z.is_empty() {
            return Err(FeagiDataProcessingError::InternalError("A given range appears invalid!".into()))
        }
        
        if x.len() == 0 || y.len() == 0 || z.len() == 0 {
            return Err(FeagiDataProcessingError::InternalError("A given range appears empty!".into()))
        }
        Ok(SingleChannelDimensionRange { x, y, z })
    }

    pub fn is_ambiguous(&self) -> bool {
        self.x.len() == 1 && self.y.len() == 1 && self.z.len() == 1
    }
    
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