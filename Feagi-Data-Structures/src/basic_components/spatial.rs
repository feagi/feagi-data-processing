//! Spatial data structures for FEAGI brain coordinates and dimensions.
//!
//! This module provides fundamental spatial primitives including 3D coordinates
//! and dimensional bounds checking for neural space representation.

use std::ops::Range;
use crate::FeagiDataError;


//region 3D Coordinates

/// 3D coordinate with unsigned 32-bit integer components.
/// Used for representing positions in neural space.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CoordinateU32 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl CoordinateU32 {
    /// Creates a new 3D coordinate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::CoordinateU32;
    /// 
    /// let coord = CoordinateU32::new(10, 20, 30);
    /// assert_eq!(coord.x, 10);
    /// assert_eq!(coord.y, 20);
    /// assert_eq!(coord.z, 30);
    /// ```
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

impl std::fmt::Display for CoordinateU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}


/// 3D coordinate with signed 32-bit integer components.
/// Used for representing relative positions or offsets in neural space.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CoordinateI32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CoordinateI32 {
    /// Creates a new 3D signed coordinate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::CoordinateI32;
    /// 
    /// let coord = CoordinateI32::new(-5, 0, 15);
    /// assert_eq!(coord.x, -5);
    /// assert_eq!(coord.y, 0);
    /// assert_eq!(coord.z, 15);
    /// ```
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl std::fmt::Display for CoordinateI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
// TODO try from for I and U 32

//endregion



//region 2D Dimensions
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct CartesianResolution {
    pub width: usize,
    pub height: usize,
}

impl std::fmt::Display for CartesianResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.width, self.height)
    }
}

impl CartesianResolution {
    pub fn new(width: usize, height: usize) -> Result<Self, FeagiDataError> {
        if width == 0 || height == 0 {
            return Err(FeagiDataError::BadParameters("Width or Height cannot be 0!".to_string()));
        }
        Ok(Self { width, height })
    }
}

//endregion



//region 3D Dimensions
/// 3D dimensions defining the bounds of neural space.
/// All dimensions must be non-zero.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Dimensions{
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Dimensions{
    /// Creates new dimensions, ensuring all values are non-zero.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::Dimensions;
    /// 
    /// let dims = Dimensions::new(100, 50, 25).unwrap();
    /// assert_eq!(dims.x, 100);
    /// 
    /// // This will fail because zero dimensions are not allowed
    /// assert!(Dimensions::new(0, 50, 25).is_err());
    /// ```
    pub fn new(x: u32, y: u32, z: u32) -> Result<Self, FeagiDataError> {
        if x == 0 || y == 0 || z == 0{
            return Err(FeagiDataError::BadParameters("Dimensions in any direction cannot be zero!".into()))
        }
        Ok(Dimensions{x, y, z})
    }
    
    /// Verifies that a coordinate falls within these dimensional bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::{Dimensions, CoordinateU32};
    /// 
    /// let dims = Dimensions::new(10, 10, 10).unwrap();
    /// let valid_coord = CoordinateU32::new(5, 7, 3);
    /// let invalid_coord = CoordinateU32::new(15, 5, 3);
    /// 
    /// assert!(dims.verify_coordinate_in_bounds(&valid_coord).is_ok());
    /// assert!(dims.verify_coordinate_in_bounds(&invalid_coord).is_err());
    /// ```
    pub fn verify_coordinate_in_bounds(&self, positive_coordinates: &CoordinateU32) -> Result<(), FeagiDataError>{
        if  positive_coordinates.x < self.x && positive_coordinates.y < self.y && positive_coordinates.z < self.z{
            return Ok(())
        }
        Err(FeagiDataError::BadParameters(format!("Point {} is not within the dimension bounds of {}!", positive_coordinates, self)))
    }
}

impl std::fmt::Display for Dimensions{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}



/// A 3D range defining acceptable coordinate bounds.
/// Each axis has its own range for flexible boundary definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DimensionRange{
    pub x: Range<u32>,
    pub y: Range<u32>,
    pub z: Range<u32>,
}

impl DimensionRange {
    /// Creates a new dimension range, ensuring no ranges are empty.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::DimensionRange;
    /// 
    /// let range = DimensionRange::new(0..10, 5..15, 0..20).unwrap();
    /// 
    /// // This will fail because the range is empty
    /// assert!(DimensionRange::new(5..5, 0..10, 0..10).is_err());
    /// ```
    pub fn new(x: Range<u32>, y: Range<u32>, z: Range<u32>) -> Result<DimensionRange, FeagiDataError> {
        if x.is_empty() || y.is_empty() || z.is_empty() {
            return Err(FeagiDataError::BadParameters("A given range has some empty or invalid ranges!".into()))
        }
        Ok(DimensionRange { x, y, z })
    }
    
    /// Returns true if any axis spans more than one value (i.e., not a single point).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::DimensionRange;
    /// 
    /// let single_point = DimensionRange::new(5..6, 10..11, 15..16).unwrap();
    /// let multi_point = DimensionRange::new(0..10, 5..6, 0..5).unwrap();
    /// 
    /// assert!(!single_point.is_ambiguous());
    /// assert!(multi_point.is_ambiguous());
    /// ```
    pub fn is_ambiguous(&self) -> bool {
        self.x.len() != 1 || self.y.len() != 1 || self.z.len() != 1
    }
    
    /// Verifies that a coordinate falls within all axis ranges.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_data_structures::basic_components::{DimensionRange, CoordinateU32};
    /// 
    /// let range = DimensionRange::new(0..10, 5..15, 0..20).unwrap();
    /// let valid_coord = CoordinateU32::new(5, 10, 15);
    /// let invalid_coord = CoordinateU32::new(15, 10, 15);
    /// 
    /// assert!(range.verify_coordinate_u32_within_range(&valid_coord).is_ok());
    /// assert!(range.verify_coordinate_u32_within_range(&invalid_coord).is_err());
    /// ```
    pub fn verify_coordinate_u32_within_range(&self, checking: &CoordinateU32) -> Result<(), FeagiDataError> {
        if !self.x.contains(&checking.x) || !self.y.contains(&checking.y) || !self.z.contains(&checking.z){
            return Err(FeagiDataError::BadParameters(format!("Point {} is not within the acceptable range of {}!", checking, self)));
        }
        Ok(())

    }
}

impl std::fmt::Display for DimensionRange{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{} - {}, {} - {}, {} - {}>", self.x.start, self.x.end, self.y.start, self.y.end, self.z.start, self.z.end)
    }
}

/*
——————————No Macros?——————————
⠀⣞⢽⢪⢣⢣⢣⢫⡺⡵⣝⡮⣗⢷⢽⢽⢽⣮⡷⡽⣜⣜⢮⢺⣜⢷⢽⢝⡽⣝
⠸⡸⠜⠕⠕⠁⢁⢇⢏⢽⢺⣪⡳⡝⣎⣏⢯⢞⡿⣟⣷⣳⢯⡷⣽⢽⢯⣳⣫⠇
⠀⠀⢀⢀⢄⢬⢪⡪⡎⣆⡈⠚⠜⠕⠇⠗⠝⢕⢯⢫⣞⣯⣿⣻⡽⣏⢗⣗⠏⠀
⠀⠪⡪⡪⣪⢪⢺⢸⢢⢓⢆⢤⢀⠀⠀⠀⠀⠈⢊⢞⡾⣿⡯⣏⢮⠷⠁⠀⠀
⠀⠀⠀⠈⠊⠆⡃⠕⢕⢇⢇⢇⢇⢇⢏⢎⢎⢆⢄⠀⢑⣽⣿⢝⠲⠉⠀⠀⠀⠀
⠀⠀⠀⠀⠀⡿⠂⠠⠀⡇⢇⠕⢈⣀⠀⠁⠡⠣⡣⡫⣂⣿⠯⢪⠰⠂⠀⠀⠀⠀
⠀⠀⠀⠀⡦⡙⡂⢀⢤⢣⠣⡈⣾⡃⠠⠄⠀⡄⢱⣌⣶⢏⢊⠂⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢝⡲⣜⡮⡏⢎⢌⢂⠙⠢⠐⢀⢘⢵⣽⣿⡿⠁⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠨⣺⡺⡕⡕⡱⡑⡆⡕⡅⡕⡜⡼⢽⡻⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣼⣳⣫⣾⣵⣗⡵⡱⡡⢣⢑⢕⢜⢕⡝⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣴⣿⣾⣿⣿⣿⡿⡽⡑⢌⠪⡢⡣⣣⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡟⡾⣿⢿⢿⢵⣽⣾⣼⣘⢸⢸⣞⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠁⠇⠡⠩⡫⢿⣝⡻⡮⣒⢽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
—————————————————————————————
 */

//endregion