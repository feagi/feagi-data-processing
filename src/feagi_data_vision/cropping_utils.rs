/// Holds pixel coordinates for cropping. Inclusive on the bottom left, exclusive on the top right
#[derive(Debug, PartialEq)]
pub struct CornerPoints {
    /// The bottom-left corner coordinate as (x, y)
    pub lower_left: (usize, usize),
    /// The top-right corner coordinate as (x, y)
    pub upper_right: (usize, usize),
}

impl CornerPoints {
    /// Creates a new CornerPoints instance
    ///
    /// # Arguments
    ///
    /// * `lower_left` - Coordinate pair (x, y) for the bottom-left corner (inclusive)
    /// * `upper_right` - Coordinate pair (x, y) for the top-right corner (exclusive)
    ///
    /// # Returns
    ///
    /// * `Result<CornerPoints, &'static str>` - A Result containing either the constructed CornerPoints
    ///   or an error message if the input coordinates are invalid
    ///
    /// # Errors
    ///
    /// Returns an error if the relative positions of the points are invalid
    pub fn new(lower_left: (usize, usize), upper_right: (usize, usize)) -> Result<CornerPoints,  &'static str> {
        if lower_left.1 >= upper_right.1 || lower_left.0 >= upper_right.0
        {
            return Err("Lower left point must be below and to the left of the upper right point!");
        }

        Ok(CornerPoints {
            lower_left,
            upper_right
        })
    }
    
    /// Checks if the defined region fits within a source frame of the given resolution
    ///
    /// # Arguments
    ///
    /// * `source_total_resolution` - The total resolution of the source frame as (width, height)
    ///
    /// # Returns
    ///
    /// * `bool` - True if the region fits within the given resolution, false otherwise
    pub fn does_fit_in_frame_of_resolution(&self, source_total_resolution: (usize, usize)) -> bool {
        return !(self.upper_right.0 > source_total_resolution.0 || self.upper_right.1 > source_total_resolution.1)
    }
    
    /// Calculates the dimensions of the area enclosed by the corner points
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - The dimensions as (width, height) of the enclosed area
    pub fn enclosed_area(&self) -> (usize, usize) {
        (self.upper_right.0 - self.lower_left.0, self.upper_right.1 - self.lower_left.1)
    }
    
    /// Gets the coordinates of the lower-right corner (Lower Inclusive, Right Exclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (x, y) for the lower-right corner
    pub fn lower_right(&self) -> (usize, usize) {
        (self.upper_right.0, self.lower_left.1)
    }
    
    /// Gets the coordinates of the upper-left corner (Upper Exclusive, Left Inclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (x, y) for the upper-left corner
    pub fn upper_left(&self) -> (usize, usize) {
        (self.lower_left.0, self.upper_right.0)
    }
}