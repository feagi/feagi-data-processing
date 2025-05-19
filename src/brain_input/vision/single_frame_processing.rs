use crate::Error::DataProcessingError;

/// Parameters for processing an image frame, including cropping, resizing, and color adjustments.
///
/// This struct holds all the parameters needed to process an image frame, with each parameter
/// being optional. The processing steps are applied in a specific order when used.
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
    /// Creates a new FrameProcessingParameters instance with all settings disabled.
    ///
    /// # Returns
    ///
    /// A new FrameProcessingParameters instance with default values:
    /// - No cropping
    /// - No resizing
    /// - No brightness adjustment
    /// - No contrast adjustment
    /// - HeightsWidthsChannels memory layout
    /// - No grayscale conversion
    /// - No color space conversion
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

    /// Clears all processing settings, resetting them to their default values.
    pub fn clear_all_settings(&mut self) {
        self.cropping_from = None;
        self.resizing_to = None;
        self.multiply_brightness_by = None;
        self.change_contrast_by = None;
        self.convert_to_grayscale = false;
        self.convert_color_space_to = None;
    }

    /// Sets the cropping region for the image.
    ///
    /// # Arguments
    ///
    /// * `cropping_from` - The CornerPoints defining the region to crop
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn set_cropping_from(&mut self, cropping_from: CornerPoints) -> &mut Self {
        self.cropping_from = Some(cropping_from);
        self
    }

    /// Sets the target resolution for resizing the image.
    ///
    /// # Arguments
    ///
    /// * `resizing_to` - The target resolution as (width, height)
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn set_resizing_to(&mut self, resizing_to: (usize, usize)) -> &mut Self {
        self.resizing_to = Some(resizing_to);
        self
    }

    /// Sets the brightness multiplication factor.
    ///
    /// # Arguments
    ///
    /// * `multiply_brightness_by` - The factor to multiply brightness by (must be positive)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(&mut Self) if the brightness factor is valid
    /// - Err(DataProcessingError) if the brightness factor is negative
    pub fn set_multiply_brightness_by(&mut self, multiply_brightness_by: f32) -> Result<&mut Self, DataProcessingError> {
        if multiply_brightness_by < 0.0 {
            return Err(DataProcessingError::InvalidInputBounds("Multiply brightness by must be positive!".into()));
        }
        self.multiply_brightness_by = Some(multiply_brightness_by);
        Ok(self)
    }

    /// Sets the contrast adjustment factor.
    ///
    /// # Arguments
    ///
    /// * `change_contrast_by` - The contrast adjustment factor between -1.0 and 1.0
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(&mut Self) if the contrast factor is valid
    /// - Err(DataProcessingError) if the contrast factor is outside the valid range
    pub fn set_change_contrast_by(&mut self, change_contrast_by: f32) -> Result<&mut Self, DataProcessingError> {
        if change_contrast_by < -1.0 || change_contrast_by > 1.0 {
            return Err(DataProcessingError::InvalidInputBounds("The contrast factor must be between -1.0 and 1.0!".into()));
        }
        self.change_contrast_by = Some(change_contrast_by);
        Ok(self)
    }
    
    /// Sets the memory layout of the source array.
    ///
    /// # Arguments
    ///
    /// * `new_source_array_ordering` - The memory layout of the input array
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(&mut Self) if the memory layout is valid
    /// - Err(DataProcessingError) if the memory layout is invalid
    pub fn set_source_array_ordering(&mut self, new_source_array_ordering: MemoryOrderLayout) -> Result<&mut Self, DataProcessingError> {
        self.memory_ordering_of_source = new_source_array_ordering;
        Ok(self)
    }

    /// Enables conversion to grayscale.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn enable_convert_to_grayscale(&mut self) -> &mut Self {
        self.convert_to_grayscale = true;
        self
    }

    /// Enables conversion to a specific color space.
    ///
    /// # Arguments
    ///
    /// * `color_space` - The target color space to convert to
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn enable_convert_to_color_space_to(&mut self, color_space: ColorSpace) -> &mut Self {
        self.convert_color_space_to = Some(color_space);
        self
    }
    
    /// Returns the cropping region if set.
    pub fn get_cropping_from(&self) -> Option<CornerPoints> {
        self.cropping_from
    }
    
    /// Returns the target resolution for resizing if set.
    pub fn get_resizing_to(&self) -> Option<(usize, usize)> {
        self.resizing_to
    }
    
    /// Returns the brightness multiplication factor if set.
    pub fn get_multiply_brightness_by(&self) -> Option<f32> {
        self.multiply_brightness_by
    }
    
    /// Returns the contrast adjustment factor if set.
    pub fn get_change_contrast_by(&self) -> Option<f32> {
        self.change_contrast_by
    }
    
    /// Returns the memory layout of the source array.
    pub fn get_memory_ordering_of_source(&self) -> MemoryOrderLayout {
        self.memory_ordering_of_source
    }
    
    /// Returns whether grayscale conversion is enabled.
    pub fn get_convert_to_grayscale(&self) -> bool {
        self.convert_to_grayscale
    }
    
    /// Returns the target color space for conversion if set.
    pub fn get_convert_to_color_space_to(&self) -> Option<ColorSpace> {
        self.convert_color_space_to
    }
    
    /// Returns a tuple indicating which processing steps are required.
    ///
    /// The tuple contains boolean flags in the following order:
    /// 1. Cropping
    /// 2. Resizing
    /// 3. Brightness adjustment
    /// 4. Contrast adjustment
    /// 5. Grayscale conversion
    /// 6. Color space conversion
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

/// Holds pixel coordinates for cropping in row-major order.
///
/// The coordinates are inclusive on the bottom-left and exclusive on the top-right.
/// In row-major order, (0,0) is at the top-left corner, with Y increasing downward
/// and X increasing rightward.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CornerPoints {
    /// The bottom-left corner coordinate as (y, x), where the top left is towards (0,0)
    lower_left: (usize, usize),
    /// The top-right corner coordinate as (y, x), where the top left is towards (0,0)
    upper_right: (usize, usize),
}

impl CornerPoints {
    /// Creates a new CornerPoints instance from row-major coordinates.
    ///
    /// # Arguments
    ///
    /// * `lower_left` - The bottom-left corner as (y, x)
    /// * `upper_right` - The top-right corner as (y, x)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(CornerPoints) if the coordinates are valid
    /// - Err(DataProcessingError) if the coordinates are invalid
    pub fn new_from_row_major_where_origin_top_left(lower_left: (usize, usize), upper_right: (usize, usize)) -> Result<CornerPoints,  DataProcessingError> {
        if lower_left.1 <= upper_right.1 || lower_left.0 >= upper_right.0 {
            return Err(DataProcessingError::InvalidInputBounds("The lower left point must have a greater Y index and a smaller X index than the upper right point!".into()));
        };
        Ok(CornerPoints {
            lower_left,
            upper_right
        })
    }

    /// Creates a new CornerPoints instance from cartesian coordinates.
    ///
    /// # Arguments
    ///
    /// * `left_lower_cartesian` - The bottom-left corner in cartesian coordinates (x, y)
    /// * `right_upper_cartesian` - The top-right corner in cartesian coordinates (x, y)
    /// * `total_resolution_width_height` - The total resolution as (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(CornerPoints) if the coordinates are valid
    /// - Err(DataProcessingError) if the coordinates are invalid or out of bounds
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

/// Represents the color space of an image.
///
/// This enum defines the possible color spaces:
/// - Linear: Linear color space
/// - Gamma: Gamma-corrected color space
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorSpace {
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

/// Represents the memory layout of an image array.
///
/// This enum defines the possible memory layouts for image data:
/// - HeightsWidthsChannels: Row-major format (default)
/// - ChannelsHeightsWidths: Common in machine learning
/// - WidthsHeightsChannels: Cartesian format
/// - HeightsChannelsWidths: Alternative format
/// - ChannelsWidthsHeights: Alternative format
/// - WidthsChannelsHeights: Alternative format
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MemoryOrderLayout {
    HeightsWidthsChannels, // default, also called row major
    ChannelsHeightsWidths, // common in machine learning
    WidthsHeightsChannels, // cartesian, the best one
    HeightsChannelsWidths,
    ChannelsWidthsHeights,
    WidthsChannelsHeights,
}

/// Converts a number of channels to a ChannelFormat.
///
/// # Arguments
///
/// * `number` - The number of color channels (1-4)
///
/// # Returns
///
/// A Result containing either:
/// - Ok(ChannelFormat) if the number of channels is valid
/// - Err(DataProcessingError) if the number of channels is invalid
pub fn usize_to_channel_format(number: usize) -> Result<ChannelFormat, DataProcessingError> {
    match number { 
        1 => Ok(ChannelFormat::GrayScale),
        2 => Ok(ChannelFormat::RG),
        3 => Ok(ChannelFormat::RGB),
        4 => Ok(ChannelFormat::RGBA),
        _ => return Err(DataProcessingError::InvalidInputBounds("The number of color channels must be at least 1 and not exceed the 4!".into()))
    }
}
