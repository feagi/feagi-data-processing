use std::ops::Range;
use crate::FeagiDataError;

//region Coordinates

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CoordinateU32 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl CoordinateU32 {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

impl std::fmt::Display for CoordinateU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CoordinateI32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CoordinateI32 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl std::fmt::Display for CoordinateI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
// TODO try from for I and U 32

//endregion

//region Dimensions
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Dimensions{
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Dimensions{
    pub fn new(x: u32, y: u32, z: u32) -> Result<Self, FeagiDataError> {
        if x == 0 || y == 0 || z == 0{
            return Err(FeagiDataError::BadParameter("Dimensions in any direction cannot be zero!".into()))
        }
        Ok(Dimensions{x, y, z})
    }
    
    pub fn verify_coordinate_in_bounds(&self, positive_coordinates: &CoordinateU32) -> Result<(), FeagiDataError>{
        if  positive_coordinates.x < self.x && positive_coordinates.y < self.y && positive_coordinates.z < self.z{
            return Ok(())
        }
        Err(FeagiDataError::BadParameter(format!("Point {} is not within the dimension bounds of {}!", positive_coordinates, self)))
    }
}

impl std::fmt::Display for Dimensions{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DimensionRange{
    pub x: Range<u32>,
    pub y: Range<u32>,
    pub z: Range<u32>,
}

impl DimensionRange {
    pub fn new(x: Range<u32>, y: Range<u32>, z: Range<u32>) -> Result<DimensionRange, FeagiDataError> {
        if x.is_empty() || y.is_empty() || z.is_empty() {
            return Err(FeagiDataError::BadParameter("A given range has some empty or invalid ranges!".into()))
        }
        Ok(DimensionRange { x, y, z })
    }
    
    pub fn is_ambiguous(&self) -> bool {
        self.x.len() != 1 || self.y.len() != 1 || self.z.len() != 1
    }
    
    pub fn verify_coordinate_u32_within_range(&self, checking: &CoordinateU32) -> Result<(), FeagiDataError> {
        if !self.x.contains(&checking.x) || !self.y.contains(&checking.y) || !self.z.contains(&checking.z){
            return Err(FeagiDataError::BadParameter(format!("Point {} is not within the acceptable range of {}!", checking, self)));
        }
        Ok(())

    }
}

impl std::fmt::Display for DimensionRange{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{} - {}, {} - {}, {} - {}>", self.x.start, self.x.end, self.y.start, self.y.end, self.z.start, self.z.end)
    }
}
//endregion