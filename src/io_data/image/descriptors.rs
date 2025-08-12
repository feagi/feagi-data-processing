//! Vision processing descriptors and parameter structures for FEAGI.
//! 
//! This module provides data structures and enums for configuring image processing
//! operations, including frame processing parameters, corner point definitions,
//! color spaces, channel formats, memory layouts, and segmented vision configurations.

use std::cmp;
use std::ops::RangeInclusive;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::ImageFrame;

//region Image Frame Properties

/// Describes the properties of an image frame including resolution, color space, and channel layout.
///
/// This struct encapsulates all the metadata needed to describe an image frame's format
/// and dimensions. It's used throughout the system for type checking, validation, and
/// ensuring compatibility between different image processing operations.
///
/// # Fields
///
/// * `xy_resolution` - The image dimensions as (width, height) in pixels
/// * `color_space` - The color space (Linear or Gamma-corrected)
/// * `color_channel_layout` - The channel configuration (Grayscale, RGB, RGBA, etc.)
///
/// # Example
///
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ChannelLayout};
///
/// let properties = ImageFrameProperties::new(
///     (640, 480),
///     ColorSpace::Linear,
///     ChannelLayout::RGB
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageFrameProperties {
    xy_resolution: (usize, usize),
    color_space: ColorSpace,
    color_channel_layout: ChannelLayout,
}

impl std::fmt::Display for ImageFrameProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = format!("ImageFrameProperties(xy_resolution: <{}, {}>, {}, {})", self.xy_resolution.0, self.xy_resolution.1, self.color_space.to_string(), self.color_channel_layout.to_string());
        write!(f, "{}", s)
    }
}

impl ImageFrameProperties {
    /// Creates a new ImageFrameProperties instance.
    ///
    /// # Arguments
    ///
    /// * `xy_resolution` - The image dimensions as (width, height) in pixels
    /// * `color_space` - The color space (Linear or Gamma-corrected)
    /// * `color_channel_layout` - The channel configuration (Grayscale, RGB, RGBA, etc.)
    ///
    /// # Returns
    ///
    /// A new ImageFrameProperties instance with the specified configuration.
    pub fn new(xy_resolution: (usize, usize), color_space: ColorSpace, color_channel_layout: ChannelLayout) -> Self {
        ImageFrameProperties{
            xy_resolution,
            color_space,
            color_channel_layout,
        }
    }

    /// Verifies that an image frame matches these properties.
    ///
    /// Checks if the given image frame has the same resolution, color space,
    /// and channel layout as specified in these properties.
    ///
    /// # Arguments
    ///
    /// * `image_frame` - The image frame to verify against these properties
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the image frame matches these properties
    /// * `Err(FeagiDataProcessingError)` if any property doesn't match
    ///
    /// # Errors
    ///
    /// Returns an error with a descriptive message if:
    /// - The resolution doesn't match
    /// - The color space doesn't match  
    /// - The channel layout doesn't match
    pub fn verify_image_frame_matches_properties(&self, image_frame: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        if image_frame.get_cartesian_width_height() != self.xy_resolution {
            return Err(IODataError::InvalidParameters(format!{"Expected resolution of <{}, {}> but received an image with resolution of <{}, {}>!",
                                                              self.xy_resolution.0, self.xy_resolution.1, image_frame.get_cartesian_width_height().0, image_frame.get_cartesian_width_height().1}).into())
        }
        if image_frame.get_color_space() != &self.color_space {
            return Err(IODataError::InvalidParameters(format!("Expected color space of {}, but got image with color space of {}!", self.color_space.to_string(), self.color_space.to_string())).into())
        }
        if image_frame.get_channel_layout() != &self.color_channel_layout {
            return Err(IODataError::InvalidParameters(format!("Expected color channel layout of {}, but got image with color channel layout of {}!", self.color_channel_layout.to_string(), self.color_channel_layout.to_string())).into())
        }
        Ok(())
    }
    
    /// Returns the expected XY resolution.
    ///
    /// # Returns
    ///
    /// A tuple containing (width, height) in pixels.
    pub fn get_expected_xy_resolution(&self) -> (usize, usize) {
        self.xy_resolution
    }
    
    /// Returns the expected color space.
    ///
    /// # Returns
    ///
    /// The ColorSpace enum value (Linear or Gamma).
    pub fn get_expected_color_space(&self) -> ColorSpace {
        self.color_space
    }
    
    /// Returns the expected color channel layout.
    ///
    /// # Returns
    ///
    /// The ChannelLayout enum value (Grayscale, RGB, RGBA, etc.).
    pub fn get_expected_color_channel_layout(&self) -> ChannelLayout {
        self.color_channel_layout
    }
}
//endregion

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
    pub fn new_from_row_major(lower_left_yx: (usize, usize), upper_right_yx: (usize, usize)) -> Result<CornerPoints,  FeagiDataProcessingError> {
        if lower_left_yx.1 >= upper_right_yx.1 {
            return Err(IODataError::InvalidParameters(format!("The lower left point must have a smaller X ({}) index than the upper right point ({})!", lower_left_yx.1, upper_right_yx.1).into()).into());
        }

        if lower_left_yx.0 <= upper_right_yx.0 {
            return Err(IODataError::InvalidParameters(format!("The lower left point must have a greater Y ({}) index than the upper right point ({})!", lower_left_yx.0, upper_right_yx.0).into()).into());
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
        -> Result<CornerPoints,  FeagiDataProcessingError> {
        
        if left_lower_xy.0 >= right_upper_xy.0 || left_lower_xy.1 >= right_upper_xy.1 {
            return Err(IODataError::InvalidParameters("Given corner points do not enclose a valid area!".into()).into());
        }
        
        if right_upper_xy.0 > total_source_resolution_width_height.0 || right_upper_xy.1 > total_source_resolution_width_height.1 {
            return Err(IODataError::InvalidParameters("Corner bounds must be within the total resolution!".into()).into());
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
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ColorSpace {
    Linear,
    Gamma
}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColorSpace::Linear => write!(f, "Linear"),
            ColorSpace::Gamma => write!(f, "Gamma"),
        }
    }
}

/// Represents the color channel format of an image.
///
/// This enum defines the possible color channel configurations for an image:
/// - GrayScale: Single channel (grayscale, or red)
/// - RG: Two channels (red, green)
/// - RGB: Three channels (red, green, blue)
/// - RGBA: Four channels (red, green, blue, alpha)
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ChannelLayout {
    GrayScale = 1, // R
    RG = 2,
    RGB = 3,
    RGBA = 4,
}

impl std::fmt::Display for ChannelLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChannelLayout::GrayScale => write!(f, "ChannelLayout(GrayScale)"),
            ChannelLayout::RG => write!(f, "ChannelLayout(RedGreen)"),
            ChannelLayout::RGB => write!(f, "ChannelLayout(RedGreenBlue)"),
            ChannelLayout::RGBA => write!(f, "ChannelLayout(RedGreenBlueAlpha)"),
        }
    }
}

impl TryFrom<usize> for ChannelLayout {
    type Error = FeagiDataProcessingError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChannelLayout::GrayScale),
            2 => Ok(ChannelLayout::RG),
            3 => Ok(ChannelLayout::RGB),
            4 => Ok(ChannelLayout::RGBA),
            _ => Err(IODataError::InvalidParameters(format!("No Channel Layout has {} channels! Acceptable values are 1,2,3,4!", value)).into())
        }
    }
}

impl From<ChannelLayout> for usize {
    fn from(value: ChannelLayout) -> usize {
        value as usize
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
pub struct SegmentedFrameCenterProperties {
    /// Center point coordinates in normalized space (0.0-1.0), from the top left
    center_coordinates_normalized_yx: (f32, f32), // Scaled from 0 to 1
    /// Size of the center region in normalized space (0.0-1.0)
    center_size_normalized_yx: (f32, f32), // ditto
}

impl SegmentedFrameCenterProperties {
    /// Creates a new SegmentedVisionCenterProperties with row-major coordinates.
    /// 
    /// This constructor creates center properties using normalized coordinates where
    /// the origin (0,0) is in the top-left corner of the image. This is typical of many toolboxes such as Numpy
    /// 
    /// # Arguments
    /// 
    /// * `center_coordinates_normalized_yx` - Center point as (y, x) in normalized space (0.0-1.0)
    /// * `center_size_normalized_yx` - Size as (height, width) in normalized space (0.0-1.0)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(SegmentedVisionCenterProperties) if the parameters are valid
    /// - Err(DataProcessingError) if coordinates or size are outside valid ranges
    pub fn new_row_major_where_origin_top_left(center_coordinates_normalized_yx: (f32, f32), center_size_normalized_yx: (f32, f32)) -> Result<SegmentedFrameCenterProperties, FeagiDataProcessingError> {
        let range_0_1: RangeInclusive<f32> = 0.0..=1.0;
        if !(range_0_1.contains(&center_coordinates_normalized_yx.0) && range_0_1.contains(&center_coordinates_normalized_yx.1)) {
            return Err(IODataError::InvalidParameters("Central vision center coordinates are to be normalized and must be between 0 and 1!".into()).into())
        }
        if !(range_0_1.contains(&center_size_normalized_yx.0) && range_0_1.contains(&center_size_normalized_yx.1)) {
            return Err(IODataError::InvalidParameters("Central vision size is to be normalized and must be between 0 and 1!".into()).into())
        }
        
        let range_overlap_y: RangeInclusive<f32> = (center_size_normalized_yx.0 / 2.0)..=(1.0 + (center_size_normalized_yx.0 / 2.0));
        let range_overlap_x: RangeInclusive<f32> = (center_size_normalized_yx.1 / 2.0)..=(1.0 + (center_size_normalized_yx.1 / 2.0));
        
        if !(range_overlap_y.contains(&center_coordinates_normalized_yx.0) && range_overlap_x.contains(&center_coordinates_normalized_yx.1)) {
            return Err(IODataError::InvalidParameters("Resulting central vision crop includes regions outside input image!".into()).into())
        }
        
        Ok(SegmentedFrameCenterProperties {
            center_coordinates_normalized_yx,
            center_size_normalized_yx,
        })
    }

    /// Creates a new SegmentedVisionCenterProperties with Cartesian coordinates.
    /// 
    /// This constructor creates center properties using normalized Cartesian coordinates
    /// where the origin (0,0) is in the bottom-left corner of the image. This is typical in graphics pipelines.
    /// 
    /// # Arguments
    /// 
    /// * `center_coordinates_normalized_cartesian_xy` - Center point as (x, y) in normalized space (0.0-1.0)
    /// * `center_size_normalized_xy` - Size as (width, height) in normalized space (0.0-1.0)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(SegmentedVisionCenterProperties) if the parameters are valid
    /// - Err(DataProcessingError) if coordinates or size are outside valid ranges
    pub fn cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy: (f32, f32), center_size_normalized_xy: (f32, f32)) -> Result<SegmentedFrameCenterProperties, FeagiDataProcessingError> {
        SegmentedFrameCenterProperties::new_row_major_where_origin_top_left(
            (center_coordinates_normalized_cartesian_xy.1, 1.0 - center_coordinates_normalized_cartesian_xy.0),
            (center_size_normalized_xy.1, center_size_normalized_xy.0))
    }

    /// Creates a default centered SegmentedFrameCenterProperties.
    /// 
    /// This convenience method creates center properties with the center region
    /// positioned at the middle of the image with a moderate size.
    /// 
    /// # Returns
    /// 
    /// A SegmentedFrameCenterProperties with default centered configuration.
    pub fn create_default_centered() -> SegmentedFrameCenterProperties {
        SegmentedFrameCenterProperties::new_row_major_where_origin_top_left((0.5, 0.5), (0.5, 0.5)).unwrap()
    }
    
    /// Calculates the source corner points for all nine segments of a segmented vision frame.
    /// 
    /// This method computes the cropping regions for each of the nine segments based on
    /// the center properties and the source frame dimensions.
    /// 
    /// # Arguments
    /// 
    /// * `source_frame_width_height` - The dimensions of the source frame as (width, height)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(SegmentedVisionFrameSourceCroppingPointGrouping) with corner points for all segments
    /// - Err(DataProcessingError) if the calculations fail
    pub(crate) fn calculate_source_corner_points_for_segmented_video_frame(&self, source_frame_width_height: (usize, usize)) -> Result<SegmentedVisionFrameSourceCroppingPointGrouping, FeagiDataProcessingError> {
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
    
    fn calculate_pixel_coordinates_of_center_corners(&self, source_frame_width_height: (usize, usize)) -> Result<CornerPoints, FeagiDataProcessingError> {
        if source_frame_width_height.0 < 3 || source_frame_width_height.1 < 3 {
            return Err(IODataError::InvalidParameters("Source resolution must be 3 pixels or greater in the X and Y directions!".into()).into());
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
pub struct SegmentedFrameTargetResolutions {
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

impl SegmentedFrameTargetResolutions {

    /// Creates a new SegmentedFrameTargetResolutions with individual segment resolutions.
    /// 
    /// This constructor allows setting different resolutions for each of the nine segments
    /// in the segmented vision frame.
    /// 
    /// # Arguments
    /// 
    /// * `lower_left` - Resolution for the lower-left segment as (width, height)
    /// * `middle_left` - Resolution for the middle-left segment as (width, height)
    /// * `upper_left` - Resolution for the upper-left segment as (width, height)
    /// * `upper_middle` - Resolution for the upper-middle segment as (width, height)
    /// * `upper_right` - Resolution for the upper-right segment as (width, height)
    /// * `middle_right` - Resolution for the middle-right segment as (width, height)
    /// * `lower_right` - Resolution for the lower-right segment as (width, height)
    /// * `lower_middle` - Resolution for the lower-middle segment as (width, height)
    /// * `center` - Resolution for the center segment as (width, height)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(SegmentedFrameTargetResolutions) if all resolutions are valid (non-zero)
    /// - Err(DataProcessingError) if any resolution has zero width or height
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
    ) -> Result<SegmentedFrameTargetResolutions, FeagiDataProcessingError> {
        if lower_left.0 == 0 || lower_left.1 == 0 || middle_left.0 == 0 || middle_left.1 == 0 || upper_left.0 == 0 || upper_left.1 == 0 || upper_middle.0 == 0 || upper_middle.1 == 0 || // Yandredev moment
            upper_right.0 == 0 || upper_right.1 == 0 || middle_right.0 == 0 || middle_right.1 == 0 || lower_right.0 == 0 || lower_right.1 == 0 || lower_middle.0 == 0 || lower_middle.1 == 0 ||
            center.0 == 0 || center.1 == 0 {
            return Err(IODataError::InvalidParameters("Dimensions must exceed 0 for all segments on all axis!".into()).into());
        }
        Ok(SegmentedFrameTargetResolutions {
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

    /// Creates a SegmentedVisionTargetResolutions with uniform peripheral segment sizes.
    /// 
    /// This convenience method creates a configuration where all eight peripheral segments
    /// have the same resolution, while the center segment can have a different resolution.
    /// 
    /// # Arguments
    /// 
    /// * `center_width_height` - Resolution for the center segment as (width, height)
    /// * `peripheral_width_height` - Resolution for all peripheral segments as (width, height)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(SegmentedVisionTargetResolutions) if all resolutions are valid (non-zero)
    /// - Err(DataProcessingError) if any resolution has zero width or height
    pub fn create_with_same_sized_peripheral(center_width_height: (usize, usize), peripheral_width_height: (usize, usize)) -> Result<SegmentedFrameTargetResolutions, FeagiDataProcessingError> {
        SegmentedFrameTargetResolutions::new(peripheral_width_height, peripheral_width_height,
                                             peripheral_width_height, peripheral_width_height,
                                             peripheral_width_height, peripheral_width_height,
                                             peripheral_width_height, peripheral_width_height,
                                             center_width_height)
    }
}

/// For internal use, convenient grouping for segmented image frame to store corner points on where from the source various regions should be cropped from
#[derive(PartialEq, Clone, Copy)]
#[derive(Debug)]
pub(crate) struct SegmentedVisionFrameSourceCroppingPointGrouping {
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
