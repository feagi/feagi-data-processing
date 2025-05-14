
use crate::Error::DataProcessingError;

#[derive(PartialEq, Clone, Copy)]
pub struct FrameProcessingParameters {
    cropping_from: Option<CornerPoints>,
    resizing_to: Option<(usize, usize)>,
    multiply_brightness_by: Option<f32>,
    change_contrast_by: Option<f32>,
    convert_to_grayscale: bool, // TODO
    convert_color_space_to: Option<ColorSpace>, // TODO
}

impl FrameProcessingParameters {
    pub fn new() -> FrameProcessingParameters {
        FrameProcessingParameters{
            cropping_from: None,
            resizing_to: None,
            multiply_brightness_by: None,
            change_contrast_by: None,
            convert_to_grayscale: false,
            convert_color_space_to: None,
        }
    }

    pub fn clear_all_settings(&mut self) {
        self.cropping_from = None;
        self.resizing_to = None;
        self.multiply_brightness_by = None;
        self.change_contrast_by = None;
        self.convert_to_grayscale = false;
        self.convert_color_space_to = None;
    }

    pub fn set_cropping_from(&mut self, cropping_from: CornerPoints) -> &mut Self {
        self.cropping_from = Some(cropping_from);
        self
    }

    pub fn set_resizing_to(&mut self, resizing_to: (usize, usize)) -> &mut Self {
        self.resizing_to = Some(resizing_to);
        self
    }

    pub fn set_multiply_brightness_by(&mut self, multiply_brightness_by: f32) -> Result<&mut Self, DataProcessingError> {
        if multiply_brightness_by < 0.0 {
            return Err(DataProcessingError::InvalidInputBounds("Multiply brightness by must be positive!".into()));
        }
        self.multiply_brightness_by = Some(multiply_brightness_by);
        Ok(self)
    }

    pub fn set_change_contrast_by(&mut self, change_contrast_by: f32) -> Result<&mut Self, DataProcessingError> {
        if change_contrast_by < -1.0 || change_contrast_by > 1.0 {
            return Err(DataProcessingError::InvalidInputBounds("The contrast factor must be between -1.0 and 1.0!".into()));
        }
        self.change_contrast_by = Some(change_contrast_by);
        Ok(self)
    }

    pub fn enable_convert_to_grayscale(&mut self) -> &mut Self {
        self.convert_to_grayscale = true;
        self
    }

    pub fn enable_convert_to_color_space_to(&mut self, color_space: ColorSpace) -> &mut Self {
        self.convert_color_space_to = Some(color_space);
        self
    }
    
    pub fn get_cropping_from(&self) -> Option<CornerPoints> {
        self.cropping_from
    }
    
    pub fn get_resizing_to(&self) -> Option<(usize, usize)> {
        self.resizing_to
    }
    
    pub fn get_multiply_brightness_by(&self) -> Option<f32> {
        self.multiply_brightness_by
    }
    
    pub fn get_change_contrast_by(&self) -> Option<f32> {
        self.change_contrast_by
    }
    
    pub fn get_convert_to_grayscale(&self) -> bool {
        self.convert_to_grayscale
    }
    
    pub fn get_convert_to_color_space_to(&self) -> Option<ColorSpace> {
        self.convert_color_space_to
    }
    
    pub fn process_steps_required_to_run(&self) -> (bool, bool, bool, bool, bool, bool) { // Yandredev moment
        (
            self.cropping_from.is_some(),
            self.resizing_to.is_some(),
            self.multiply_brightness_by.is_some(),
            self.change_contrast_by.is_some(),
            self.convert_to_grayscale,
            self.convert_color_space_to.is_some(),
            )
    }
}



/// Holds pixel coordinates for cropping. Inclusive on the bottom left, exclusive on the top right
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CornerPoints {
    /// The bottom-left corner coordinate as (x, y)
    lower_left: (usize, usize),
    /// The top-right corner coordinate as (x, y)
    upper_right: (usize, usize),
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
    pub fn new(lower_left: (usize, usize), upper_right: (usize, usize)) -> Result<CornerPoints,  DataProcessingError> {
        if lower_left.1 >= upper_right.1 || lower_left.0 >= upper_right.0
        {
            return Err(DataProcessingError::InvalidInputBounds("Lower left point must be below and to the left of the upper right point!".into()));
        }

        Ok(CornerPoints {
            lower_left,
            upper_right
        })
    }

    /// Gets the coordinates of the lower-left corner (Lower Inclusive, Left Inclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (x, y) for the lower-left corner
    pub fn lower_left(&self) -> (usize, usize) {
        self.lower_left
    }

    /// Gets the coordinates of the upper-right corner (Upper Exclusive, Right Exclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (x, y) for the upper-right corner
    pub fn upper_right(&self) -> (usize, usize) {
        self.upper_right
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
        self.upper_right.0 <= source_total_resolution.0 || self.upper_right.1 <= source_total_resolution.1
    }

    /// Calculates the dimensions of the area enclosed by the corner points
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - The dimensions as (width, height) of the enclosed area
    pub fn enclosed_area(&self) -> (usize, usize) {
        (self.upper_right.0 - self.lower_left.0, self.upper_right.1 - self.lower_left.1)
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorSpace{
    Linear,
    Gamma
}

/// Represents the color channel format of an image.
///
/// This enum defines the possible color channel configurations for an image:
/// - GrayScale: Single channel (grayscale, or red)
/// - RG: Two channels (red, green)
/// - RGB: Three channels (red, green, blue)
/// - RGBA: Four channels (red, green, blue, alpha)
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChannelFormat {
    GrayScale = 1, // R
    RG = 2,
    RGB = 3,
    RGBA = 4,
}