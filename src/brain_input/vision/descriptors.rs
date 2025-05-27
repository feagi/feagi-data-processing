use std::cmp;
use std::ops::RangeInclusive;
use crate::error::DataProcessingError;



/// Parameters for processing an image frame, including cropping, resizing, and color adjustments.
///
/// This struct holds all the parameters needed to process an image frame, with each parameter
/// being optional. The processing steps are applied in a specific order when used.
#[derive(PartialEq, Clone, Copy)]
pub struct FrameProcessingParameters {
    pub cropping_from: Option<CornerPoints>,
    pub resizing_to: Option<(usize, usize)>,
    pub multiply_brightness_by: Option<f32>,
    pub change_contrast_by: Option<f32>,
    pub memory_ordering_of_source: MemoryOrderLayout,
    pub convert_to_grayscale: bool, // TODO
    pub convert_color_space_to: Option<ColorSpace>, // TODO
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
    
    pub fn get_final_width_height(&self) -> Result<(usize, usize), DataProcessingError> {
        let crop_resize_exist = (self.cropping_from.is_some(), self.resizing_to.is_some());
        match crop_resize_exist {
            (false, false) => Err(DataProcessingError::MissingContext("Unknown final width of height as its not being changed by the image preprocessor!".into())),
            (true, false) => Ok(self.cropping_from.unwrap().enclosed_area_width_height()),
            _ => Ok(self.resizing_to.unwrap()),
        }
    }
}

/// Holds pixel coordinates for cropping in row-major order.
///
/// The coordinates are inclusive on the bottom-left and exclusive on the top-right.
/// In row-major order, (0,0) is in the top-left corner, with Y increasing downward
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
    /// * `lower_left_yx` - The bottom-left corner as (y, x)
    /// * `upper_right_ux` - The top-right corner as (y, x)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(CornerPoints) if the coordinates are valid
    /// - Err(DataProcessingError) if the coordinates are invalid
    pub fn new_from_row_major(lower_left_yx: (usize, usize), upper_right_yx: (usize, usize)) -> Result<CornerPoints,  DataProcessingError> {
        if lower_left_yx.1 < upper_right_yx.1 || lower_left_yx.0 > upper_right_yx.0 {
            return Err(DataProcessingError::InvalidInputBounds("The lower left point must have a greater Y index and a smaller X index than the upper right point!".into()));
        };
        Ok(CornerPoints {
            lower_left: lower_left_yx,
            upper_right: upper_right_yx 
        })
    }

    /// Creates a new CornerPoints instance from cartesian coordinates.
    ///
    /// # Arguments
    ///
    /// * `left_lower_xy` - The bottom-left corner in cartesian coordinates (x, y)
    /// * `right_upper_xy` - The top-right corner in cartesian coordinates (x, y)
    /// * `total_source_resolution_width_height` - The total resolution as (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(CornerPoints) if the coordinates are valid
    /// - Err(DataProcessingError) if the coordinates are invalid or out of bounds
    pub fn new_from_cartesian(
        left_lower_xy: (usize, usize), right_upper_xy: (usize, usize),
        total_source_resolution_width_height: (usize, usize))
        -> Result<CornerPoints,  DataProcessingError> {
        if left_lower_xy.0 > total_source_resolution_width_height.0 || right_upper_xy.0 > total_source_resolution_width_height.0 ||
            left_lower_xy.1 > total_source_resolution_width_height.1 || right_upper_xy.1 > total_source_resolution_width_height.1 {
            return Err(DataProcessingError::InvalidInputBounds("Corner bounds must be within the total resolution!".into()));
        }
        
        Ok(CornerPoints {
            lower_left: (total_source_resolution_width_height.1 - left_lower_xy.1, left_lower_xy.0),
            upper_right: (total_source_resolution_width_height.1 - right_upper_xy.1, right_upper_xy.0)
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
        self.upper_right.1 <= width_height.0 && self.lower_left.0 <= width_height.1
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

impl ChannelFormat {
    pub fn from_usize(val: usize) -> Result<ChannelFormat, DataProcessingError> {
        match val {
            1 => Ok(ChannelFormat::GrayScale),
            2 => Ok(ChannelFormat::RG),
            3 => Ok(ChannelFormat::RGB),
            4 => Ok(ChannelFormat::RGBA),
            _ => Err(DataProcessingError::InvalidInputBounds("The number of color channels must be at least 1 and not exceed the 4!".into()))
        }
    }
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

/// Properties defining the center region of a segmented vision frame
///
/// This structure defines the coordinates and size of the central region
/// in a normalized coordinate space (0.0 to 1.0).
#[derive(PartialEq, Clone, Copy)]
pub struct SegmentedVisionCenterProperties {
    /// Center point coordinates in normalized space (0.0-1.0), from the top left
    center_coordinates_normalized_yx: (f32, f32), // Scaled from 0 to 1
    /// Size of the center region in normalized space (0.0-1.0)
    center_size_normalized_yx: (f32, f32), // ditto
}

impl SegmentedVisionCenterProperties {
    pub fn new_row_major_where_origin_top_left(center_coordinates_normalized_yx: (f32, f32), center_size_normalized_yx: (f32, f32)) -> Result<SegmentedVisionCenterProperties, DataProcessingError> {
        let range_0_1: RangeInclusive<f32> = 0.0..=1.0;
        if !(range_0_1.contains(&center_coordinates_normalized_yx.0) && range_0_1.contains(&center_coordinates_normalized_yx.1)) {
            return Err(DataProcessingError::InvalidInputBounds("Central vision center coordinates are to be normalized and must be between 0 and 1!".into()))
        }
        if !(range_0_1.contains(&center_size_normalized_yx.0) && range_0_1.contains(&center_size_normalized_yx.1)) {
            return Err(DataProcessingError::InvalidInputBounds("Central vision size is to be normalized and must be between 0 and 1!".into()))
        }
        
        let range_overlap_y: RangeInclusive<f32> = (center_size_normalized_yx.0 / 2.0)..=(1.0 + (center_size_normalized_yx.0 / 2.0));
        let range_overlap_x: RangeInclusive<f32> = (center_size_normalized_yx.1 / 2.0)..=(1.0 + (center_size_normalized_yx.1 / 2.0));
        
        if !(range_overlap_y.contains(&center_coordinates_normalized_yx.0) && range_overlap_x.contains(&center_coordinates_normalized_yx.1)) {
            return Err(DataProcessingError::InvalidInputBounds("Resulting central vision crop includes regions outside input image!".into()))
        }
        
        Ok(SegmentedVisionCenterProperties {
            center_coordinates_normalized_yx,
            center_size_normalized_yx,
        })
    }

    pub fn cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy: (f32, f32), center_size_normalized_xy: (f32, f32)) -> Result<SegmentedVisionCenterProperties, DataProcessingError> {
        SegmentedVisionCenterProperties::new_row_major_where_origin_top_left(
            (center_coordinates_normalized_cartesian_xy.1, 1.0 - center_coordinates_normalized_cartesian_xy.0),
            (center_size_normalized_xy.1, center_size_normalized_xy.0))
    }

    pub fn create_default_centered() -> SegmentedVisionCenterProperties {
        SegmentedVisionCenterProperties::new_row_major_where_origin_top_left((0.5, 0.5), (0.5, 0.5)).unwrap()
    }
    
    pub fn calculate_source_corner_points_for_segemented_video_frame(&self, source_frame_width_height: (usize, usize))  -> Result<SegmentedVisionFrameSourceCroppingPointGrouping, DataProcessingError> {
        let center_corner_points = self.calculate_pixel_coordinates_of_center_corners(source_frame_width_height)?;
        Ok(SegmentedVisionFrameSourceCroppingPointGrouping{
            lower_left: CornerPoints::new_from_row_major((source_frame_width_height.1, 0), center_corner_points.lower_left_row_major())?,
            middle_left: CornerPoints::new_from_row_major((center_corner_points.lower_left_row_major().0, 0), center_corner_points.upper_left_row_major())?,
            upper_left: CornerPoints::new_from_row_major((center_corner_points.upper_right_row_major().0, 0), (0, center_corner_points.lower_left_row_major().1))?,
            upper_middle: CornerPoints::new_from_row_major(center_corner_points.upper_left_row_major(), (0, center_corner_points.upper_right_row_major().1))?,
            upper_right: CornerPoints::new_from_row_major(center_corner_points.upper_right_row_major(), (0, source_frame_width_height.0))?,
            middle_right: CornerPoints::new_from_row_major(center_corner_points.lower_right_row_major(), (center_corner_points.upper_right_row_major().0, source_frame_width_height.0))?,
            lower_right: CornerPoints::new_from_row_major((source_frame_width_height.1, center_corner_points.upper_right_row_major().1), (center_corner_points.lower_left_row_major().1, source_frame_width_height.0))?,
            lower_middle: CornerPoints::new_from_row_major((source_frame_width_height.1, center_corner_points.lower_left_row_major().1), center_corner_points.lower_right_row_major())?,
            center: center_corner_points
        })
    }
    
    fn calculate_pixel_coordinates_of_center_corners(&self, source_frame_width_height: (usize, usize)) -> Result<CornerPoints, DataProcessingError> {
        if source_frame_width_height.0 < 3 || source_frame_width_height.1 < 3 {
            return Err(DataProcessingError::InvalidInputBounds("Source resolution must be 3 pixels or greater in the X and Y directions!".into()));
        }
        let source_frame_width_height_f: (f32, f32) = (source_frame_width_height.0 as f32, source_frame_width_height.1 as f32);
        let center_size_normalized_half_yx: (f32, f32) = (self.center_size_normalized_yx.0 / 2.0, self.center_size_normalized_yx.1 / 2.0);

        // We use max / min to ensure that there is always a 1 pixel buffer along all edges for use in peripheral vision (since we cannot use a resolution of 0)
        let bottom_pixel: usize = cmp::min(source_frame_width_height.0 - 1,
                                           ((self.center_coordinates_normalized_yx.0 + center_size_normalized_half_yx.0) * source_frame_width_height_f.1).ceil() as usize);
        let top_pixel: usize = cmp::max(1,
                                        (( self.center_coordinates_normalized_yx.0 - center_size_normalized_half_yx.0) * source_frame_width_height_f.1).floor() as usize);
        let left_pixel: usize = cmp::max(1,
                                         ((self.center_coordinates_normalized_yx.1 - center_size_normalized_half_yx.1) * source_frame_width_height_f.0).floor() as usize);
        let right_pixel: usize = cmp::min(source_frame_width_height.0 - 1,
                                          (( self.center_coordinates_normalized_yx.1 + center_size_normalized_half_yx.1) * source_frame_width_height_f.0).ceil() as usize);

        let corner_points: CornerPoints = CornerPoints::new_from_row_major(
            (bottom_pixel, left_pixel),
            (top_pixel, right_pixel)
        )?;
        Ok(corner_points)
    }
    
}

/// Target resolutions for each of the nine segments in a segmented vision frame
///
/// This structure stores the desired output resolution for each of the segments
/// in a grid arrangement (3x3): corners, edges, and center.
#[derive(PartialEq, Clone, Copy)]
pub struct SegmentedVisionTargetResolutions {
    /// Resolution for lower-left segment as (width, height)
    pub lower_left: (usize, usize),
    /// Resolution for middle-left segment as (width, height)
    pub middle_left: (usize, usize),
    /// Resolution for upper-left segment as (width, height)
    pub upper_left: (usize, usize),
    /// Resolution for upper-middle segment as (width, height)
    pub upper_middle: (usize, usize),
    /// Resolution for upper-right segment as (width, height)
    pub upper_right: (usize, usize),
    /// Resolution for middle-right segment as (width, height)
    pub middle_right: (usize, usize),
    /// Resolution for lower-right segment as (width, height)
    pub lower_right: (usize, usize),
    /// Resolution for lower-middle segment as (width, height)
    pub lower_middle: (usize, usize),
    /// Resolution for center segment as (width, height)
    pub center: (usize, usize),
}

impl SegmentedVisionTargetResolutions {

    pub fn new(
        lower_left: (usize, usize),
        middle_left: (usize, usize),
        upper_left: (usize, usize),
        upper_middle: (usize, usize),
        upper_right: (usize, usize),
        middle_right: (usize, usize),
        lower_right: (usize, usize),
        lower_middle: (usize, usize),
        center: (usize, usize),
    ) -> Result<SegmentedVisionTargetResolutions, DataProcessingError> {
        if lower_left.0 == 0 || lower_left.1 == 0 || middle_left.0 == 0 || middle_left.1 == 0 || upper_left.0 == 0 || upper_left.1 == 0 || upper_middle.0 == 0 || upper_middle.1 == 0 || // Yandre-dev moment
            upper_right.0 == 0 || upper_right.1 == 0 || middle_right.0 == 0 || middle_right.1 == 0 || lower_right.0 == 0 || lower_right.1 == 0 || lower_middle.0 == 0 || lower_middle.1 == 0 ||
            center.0 == 0 || center.1 == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Dimensions must exceed 0 for all segments on all axis!".into()));
        }
        Ok(SegmentedVisionTargetResolutions {
            lower_left,
            middle_left,
            upper_left,
            upper_middle,
            upper_right,
            middle_right,
            lower_right,
            lower_middle,
            center,
        })
    }

    pub fn create_with_same_sized_peripheral(center_width_height: (usize, usize), peripheral_width_height: (usize, usize)) -> Result<SegmentedVisionTargetResolutions, DataProcessingError> {
        SegmentedVisionTargetResolutions::new(peripheral_width_height, peripheral_width_height,
                                                     peripheral_width_height, peripheral_width_height,
                                                     peripheral_width_height, peripheral_width_height,
                                                     peripheral_width_height, peripheral_width_height,
                                                     center_width_height)
    }
}

/// For internal use, convenient grouping for segmented image frame to store corner points on where from the source various regions should be cropped from
#[derive(PartialEq, Clone, Copy)]
pub struct SegmentedVisionFrameSourceCroppingPointGrouping {
    /// Corner points for the lower-left segment
    pub lower_left: CornerPoints,
    /// Corner points for the middle-left segment
    pub middle_left: CornerPoints,
    /// Corner points for the upper-left segment
    pub upper_left: CornerPoints,
    /// Corner points for the upper-middle segment
    pub upper_middle: CornerPoints,
    /// Corner points for the upper-right segment
    pub upper_right: CornerPoints,
    /// Corner points for the middle-right segment
    pub middle_right: CornerPoints,
    /// Corner points for the lower-right segment
    pub lower_right: CornerPoints,
    /// Corner points for the lower-middle segment
    pub lower_middle: CornerPoints,
    /// Corner points for the center segment
    pub center: CornerPoints,
}
