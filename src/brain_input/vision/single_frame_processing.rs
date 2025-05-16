
use crate::Error::DataProcessingError;

#[derive(PartialEq, Clone, Copy)]
pub struct FrameProcessingParameters {
    cropping_from: Option<CornerPoints>,
    resizing_to: Option<(usize, usize)>,
    multiply_brightness_by: Option<f32>,
    change_contrast_by: Option<f32>,
    memory_ordering_of_source: MemoryOrderLayout,
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
            memory_ordering_of_source: MemoryOrderLayout::HeightsWidthsChannels,
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
    
    pub fn set_source_array_ordering(&mut self, new_source_array_ordering: MemoryOrderLayout) -> Result<&mut Self, DataProcessingError> {
        self.memory_ordering_of_source = new_source_array_ordering;
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
    
    pub fn get_memory_ordering_of_source(&self) -> MemoryOrderLayout {
        self.memory_ordering_of_source
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



/// Holds pixel coordinates for cropping. Inclusive on the bottom left, exclusive on the top right. Holds in Row Major order
/// Corners are labeled as if the row major pixels are arranged such that 0,0 is in the top left
/// corner, and Y increases downward and x increases rightward
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CornerPoints {
    /// The bottom-left corner coordinate as (y, x), where the top left is towards (0,0)
    lower_left: (usize, usize),
    /// The top-right corner coordinate as (y, x), where the top left is towards (0,0)
    upper_right: (usize, usize),
}

impl CornerPoints {

    pub fn new_from_row_major_where_origin_top_left(lower_left: (usize, usize), upper_right: (usize, usize)) -> Result<CornerPoints,  DataProcessingError> {
        if lower_left.1 <= upper_right.1 || lower_left.0 >= upper_right.0 {
            return Err(DataProcessingError::InvalidInputBounds("The lower left point must have a greater Y index and a smaller X index than the upper right point!".into()));
        };
        Ok(CornerPoints {
            lower_left,
            upper_right
        })
    }

    pub fn new_from_cartesian_where_origin_bottom_left(
        left_lower_cartesian: (usize, usize), right_upper_cartesian: (usize, usize),
        total_resolution_width_height: (usize, usize))
        -> Result<CornerPoints,  DataProcessingError> {
        if left_lower_cartesian.0 > total_resolution_width_height.0 || right_upper_cartesian.0 > total_resolution_width_height.0 ||
            left_lower_cartesian.1 > total_resolution_width_height.1 || right_upper_cartesian.1 > total_resolution_width_height.1 {
            return Err(DataProcessingError::InvalidInputBounds("Corner bounds must be within the total resolution!".into()));
        }
        
        Ok(CornerPoints {
            lower_left: (total_resolution_width_height.1 - left_lower_cartesian.1, left_lower_cartesian.0),
            upper_right: (total_resolution_width_height.1 - right_upper_cartesian.1, right_upper_cartesian.0)
        })
    }
    
    

    /// Gets the row major coordinates of the lower-left corner (Lower Exclusive, Left Inclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (y, x) for the lower-left corner
    pub fn lower_left_row_major(&self) -> (usize, usize) {
        self.lower_left
    }

    /// Gets the row major coordinates of the upper-right corner (Upper Inclusive, Right Exclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (y, x) for the upper-right corner
    pub fn upper_right_row_major(&self) -> (usize, usize) {
        self.upper_right
    }

    /// Gets the row major coordinates of the lower-right corner (Lower Exclusive, Right Exclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (y, x) for the lower-right corner
    pub fn lower_right_row_major(&self) -> (usize, usize) {
        (self.lower_left.0, self.upper_right.1)
    }

    /// Gets the row major coordinates of the upper-left corner (Upper Inclusive, Left Inclusive)
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - Coordinate pair (y, x) for the upper-left corner
    pub fn upper_left_row_major(&self) -> (usize, usize) {
        (self.upper_right.0, self.lower_left.1)
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
    pub fn does_fit_in_frame_of_width_height(&self, width_height: (usize, usize)) -> bool {
        self.upper_right.1 <= width_height.0 || self.lower_left.0 <= width_height.1
    }

    /// Calculates the dimensions of the area enclosed by the corner points
    ///
    /// # Returns
    ///
    /// * `(usize, usize)` - The dimensions as (width, height) of the enclosed area
    pub fn enclosed_area_width_height(&self) -> (usize, usize) {
        (self.upper_right.1 - self.lower_left.1, self.lower_left.0 - self.upper_right.0)
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MemoryOrderLayout {
    HeightsWidthsChannels, // default, also called row major
    ChannelsHeightsWidths, // common in machine learning
    WidthsHeightsChannels, // cartesian, the best one
    HeightsChannelsWidths,
    ChannelsWidthsHeights,
    WidthsChannelsHeights,
}

pub fn usize_to_channel_format(number: usize) -> Result<ChannelFormat, DataProcessingError> {
    match number { 
        1 => Ok(ChannelFormat::GrayScale),
        2 => Ok(ChannelFormat::RG),
        3 => Ok(ChannelFormat::RGB),
        4 => Ok(ChannelFormat::RGBA),
        _ => return Err(DataProcessingError::InvalidInputBounds("The number of color channels must be at least 1 and not exceed the 4!".into()))
    }
}

