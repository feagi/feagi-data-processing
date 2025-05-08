use super::single_frame::ImageFrame;
use super::cropping_utils::CornerPoints;
use std::cmp;

/// Properties defining the center region of a segmented vision frame
/// 
/// This structure defines the coordinates and size of the central region
/// in a normalized coordinate space (0.0 to 1.0).
#[derive(PartialEq)]
pub struct SegmentedVisionCenterProperties {
    /// Center point coordinates in normalized space (0.0-1.0)
    center_coordinates_normalized: (f32, f32), // Scaled from 0 - 1
    /// Size of the center region in normalized space (0.0-1.0)
    center_size_normalized: (f32, f32), // ditto
}

impl SegmentedVisionCenterProperties {
    /// Creates a new instance with normalized center coordinates and size
    ///
    /// # Arguments
    ///
    /// * `center_coordinates_normalized` - Center point as (x, y) in normalized space (0.0-1.0)
    /// * `center_size_normalized` - Size as (width, height) in normalized space (0.0-1.0)
    ///
    /// # Returns
    ///
    /// * `Result<SegmentedVisionCenterProperties, &'static str>` - Created instance or error message
    ///
    /// # Errors
    ///
    /// Returns an error if any coordinate or size value is outside the range [0.0, 1.0]
    pub fn new(center_coordinates_normalized: (f32, f32), center_size_normalized: (f32, f32)) -> Result<SegmentedVisionCenterProperties, &'static str> {
        if center_coordinates_normalized.0 < 0.0 || center_coordinates_normalized.1 < 0.0 || center_coordinates_normalized.0 > 1.0 || center_coordinates_normalized.1 > 1.0 ||
            center_size_normalized.0 < 0.0 || center_size_normalized.1 < 0.0 || center_size_normalized.0 > 1.0 || center_size_normalized.1 > 1.0 {
            return Err("Coordinates are to be normalized and must be between 0 and 1");
        }
        Ok(SegmentedVisionCenterProperties {
            center_coordinates_normalized,
            center_size_normalized,
        })
    }
    
    /// Returns a tuple of tuple XY coordinates representing in pixel space the corners of the central image frame, in order of lower left, top left, top right, lower right
    /// 
    /// # Arguments
    ///
    /// * `source_frame_resolution` - The total resolution (width, height) of the source frame in pixels
    ///
    /// # Returns
    ///
    /// * `Result<CornerPoints, &'static str>` - Corner points that define the center region or an error message
    ///
    /// # Errors
    ///
    /// Returns an error if the source resolution is less than 3x3 pixels
    pub fn calculate_pixel_coordinates_of_center_corners(&self, source_frame_resolution: (usize, usize)) -> Result<CornerPoints, &'static str> {
        if source_frame_resolution.0 < 3 || source_frame_resolution.1 < 3 {
            return Err("Source resolution must be 3 pixels or greater in the X and Y directions!");
        }
        let source_frame_resolution_f: (f32, f32) = (source_frame_resolution.0 as f32, source_frame_resolution.1 as f32);
        let center_size_normalized_half: (f32, f32) = (self.center_size_normalized.0 / 2.0, self.center_size_normalized.1 / 2.0);
        
        // We use max / min to ensure that there is always a 1 pixel buffer along all edges for use in peripheral vision (since we cannot use a resolution of 0)
        let bottom_pixel: usize = cmp::max(1, ((self.center_coordinates_normalized.1 - center_size_normalized_half.1) * source_frame_resolution_f.1).floor() as usize);
        let top_pixel: usize = cmp::min(source_frame_resolution.1 - 1, (( self.center_coordinates_normalized.1 + center_size_normalized_half.1) * source_frame_resolution_f.1).ceil() as usize);
        let left_pixel: usize = cmp::max(1, ((self.center_coordinates_normalized.0 - center_size_normalized_half.0) * source_frame_resolution_f.0).floor() as usize);
        let right_pixel: usize = cmp::min(source_frame_resolution.0 - 1, (( self.center_coordinates_normalized.0 + center_size_normalized_half.0) * source_frame_resolution_f.0).ceil() as usize);
        let corner_points: CornerPoints = CornerPoints::new((left_pixel, bottom_pixel), (right_pixel, top_pixel)).unwrap(); // We know that this input will not fail due to earlier checks
        Ok(corner_points)
    }
    
    
}

/// Target resolutions for each of the nine segments in a segmented vision frame
///
/// This structure stores the desired output resolution for each of the segments
/// in a grid arrangement (3x3): corners, edges, and center.
#[derive(PartialEq)]
pub struct SegmentedVisionTargetResolutions {
    /// Resolution for lower-left segment as (width, height)
    lower_left: (usize, usize),
    /// Resolution for middle-left segment as (width, height)
    middle_left: (usize, usize),
    /// Resolution for upper-left segment as (width, height)
    upper_left: (usize, usize),
    /// Resolution for upper-middle segment as (width, height)
    upper_middle: (usize, usize),
    /// Resolution for upper-right segment as (width, height)
    upper_right: (usize, usize),
    /// Resolution for middle-right segment as (width, height)
    middle_right: (usize, usize),
    /// Resolution for lower-right segment as (width, height)
    lower_right: (usize, usize),
    /// Resolution for lower-middle segment as (width, height)
    lower_middle: (usize, usize),
    /// Resolution for center segment as (width, height)
    center: (usize, usize),
}

impl SegmentedVisionTargetResolutions {
    /// Creates a new SegmentedVisionTargetResolutions with specified resolutions for each segment
    ///
    /// # Arguments
    ///
    /// * Nine pairs of (width, height) values, one for each segment
    ///
    /// # Returns
    ///
    /// * `Result<SegmentedVisionTargetResolutions, &'static str>` - Created instance or error message
    ///
    /// # Errors
    ///
    /// Returns an error if any dimension is zero
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
    ) -> Result<SegmentedVisionTargetResolutions, &'static str> {
        if lower_left.0 == 0 || lower_left.1 == 0 || middle_left.0 == 0 || middle_left.1 == 0 || upper_left.0 == 0 || upper_left.1 == 0 || upper_middle.0 == 0 || upper_middle.1 == 0 || // Yandre-dev moment
            upper_right.0 == 0 || upper_right.1 == 0 || middle_right.0 == 0 || middle_right.1 == 0 || lower_right.0 == 0 || lower_right.1 == 0 || lower_middle.0 == 0 || lower_middle.1 == 0 ||
            center.0 == 0 || center.1 == 0 {
            return Err("Dimensions must exceed 0 for all segments on all axis!");
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
}

/// A frame divided into nine segments with different resolutions. Used for Peripheral vision in FEAGI
///
/// This structure holds nine image frames representing a segmented view
/// of a source frame, with a high-resolution center and lower-resolution
/// peripheral regions.
pub struct SegmentedVisionFrame {
    /// Lower-left segment of the vision frame
    lower_left: ImageFrame,
    /// Middle-left segment of the vision frame
    middle_left: ImageFrame,
    /// Upper-left segment of the vision frame
    upper_left: ImageFrame,
    /// Upper-middle segment of the vision frame
    upper_middle: ImageFrame,
    /// Upper-right segment of the vision frame
    upper_right: ImageFrame,
    /// Middle-right segment of the vision frame
    middle_right: ImageFrame,
    /// Lower-right segment of the vision frame
    lower_right: ImageFrame,
    /// Lower-middle segment of the vision frame
    lower_middle: ImageFrame,
    /// Center segment of the vision frame (typically higher resolution)
    center: ImageFrame,
    /// Resolution of the original source frame
    original_source_resolution: (usize, usize),
    /// Corner points defining the boundaries of each segment
    segment_corner_points: SegmentedCornerPoints,
}

impl SegmentedVisionFrame {
    /// Creates a new SegmentedVisionFrame from a source frame, center properties, and target resolutions
    ///
    /// # Arguments
    ///
    /// * `source_frame` - The source image frame to segment
    /// * `center_properties` - Properties defining the center region of interest
    /// * `segment_resolutions` - Target resolutions for each segment
    ///
    /// # Returns
    ///
    /// * `Result<SegmentedVisionFrame, &'static str>` - Created instance or error message
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Calculation of center corner points fails
    /// * Creation of segment corner points fails
    pub fn new(source_frame: &ImageFrame, center_properties: &SegmentedVisionCenterProperties, segment_resolutions: SegmentedVisionTargetResolutions) -> Result<SegmentedVisionFrame, &'static str> {
        let source_frame_resolution: (usize, usize) = source_frame.get_xy_resolution();
        let inner_corners = center_properties.calculate_pixel_coordinates_of_center_corners(source_frame_resolution);
        if inner_corners.is_err(){
            return Err(inner_corners.unwrap_err()); // lol
        }
        let inner_corners: CornerPoints = inner_corners.unwrap();
        let segment_corner_points = SegmentedCornerPoints::from_source_and_center_corner_points(source_frame_resolution, inner_corners);
        if segment_corner_points.is_err(){
            return Err(segment_corner_points.unwrap_err()); // lol x2
        }
        let segment_corner_points: SegmentedCornerPoints = segment_corner_points.unwrap();
        
        // For all the following, we know the crops are safe
        Ok(SegmentedVisionFrame{
            lower_left: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.lower_left, &segment_resolutions.lower_left).unwrap(),
            middle_left: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.middle_left, &segment_resolutions.middle_left).unwrap(),
            upper_left: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.upper_left, &segment_resolutions.upper_left).unwrap(),
            upper_middle: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.upper_middle, &segment_resolutions.upper_middle).unwrap(),
            upper_right: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.upper_right, &segment_resolutions.upper_right).unwrap(),
            middle_right: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.middle_right, &segment_resolutions.middle_right).unwrap(),
            lower_right: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.lower_right, &segment_resolutions.lower_right).unwrap(),
            lower_middle: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.lower_middle, &segment_resolutions.lower_middle).unwrap(),
            center: ImageFrame::from_source_frame_crop_and_resize(source_frame, &segment_corner_points.center, &segment_resolutions.center).unwrap(),
            original_source_resolution: source_frame_resolution,
            segment_corner_points
        })
    }
    
    /// Updates the segmentation with a new focus point while using the same source frame
    ///
    /// # Arguments
    ///
    /// * `source_frame` - The source image frame
    /// * `center_properties` - New properties defining the center region of interest
    ///
    /// # Returns
    ///
    /// * `Result<(), &'static str>` - Success or error message
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The resolution of the new frame doesn't match the original
    /// * Calculation of corner points fails
    pub fn update_in_place_new_focus(&mut self, source_frame: &ImageFrame, center_properties: &SegmentedVisionCenterProperties) -> Result<(), &'static str> {
        if source_frame.get_xy_resolution() != self.original_source_resolution {
            return Err("New frame does not have the same resolution as the previous! Unable to update in place!");
        }
        
        let inner_corners = center_properties.calculate_pixel_coordinates_of_center_corners(self.original_source_resolution);
        if inner_corners.is_err(){
            return Err(inner_corners.unwrap_err()); // lol
        }
        let inner_corners: CornerPoints = inner_corners.unwrap();
        let segment_corner_points = SegmentedCornerPoints::from_source_and_center_corner_points(source_frame.get_xy_resolution(), inner_corners);
        if segment_corner_points.is_err(){
            return Err(segment_corner_points.unwrap_err()); // lol x2
        }
        
        self.segment_corner_points = segment_corner_points.unwrap();
        _ = self.lower_left.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.lower_left, source_frame);
        _ = self.middle_left.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.middle_left, source_frame);
        _ = self.upper_left.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.upper_left, source_frame);
        _ = self.upper_middle.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.upper_middle, source_frame);
        _ = self.upper_right.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.upper_right, source_frame);
        _ = self.middle_right.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.middle_right, source_frame);
        _ = self.lower_right.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.lower_right, source_frame);
        _ = self.lower_middle.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.lower_middle, source_frame);
        _ = self.center.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.center, source_frame);
        
        Ok(())
    }
    
    /// Updates all segments with a new source frame while maintaining the same focus point and size
    ///
    /// # Arguments
    ///
    /// * `source_frame` - The new source image frame
    ///
    /// # Returns
    ///
    /// * `Result<(), &'static str>` - Success or error message
    ///
    /// # Errors
    ///
    /// Returns an error if the resolution of the new frame doesn't match the original
    pub fn update_in_place(&mut self, source_frame: &ImageFrame) -> Result<(), &'static str> {
        
        if source_frame.get_xy_resolution() != self.original_source_resolution {
            return Err("New frame does not have the same resolution as the previous! Unable to update in place!");
        }
        
        _ = self.lower_left.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.lower_left, source_frame);
        _ = self.middle_left.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.middle_left, source_frame);
        _ = self.upper_left.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.upper_left, source_frame);
        _ = self.upper_middle.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.upper_middle, source_frame);
        _ = self.upper_right.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.upper_right, source_frame);
        _ = self.middle_right.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.middle_right, source_frame);
        _ = self.lower_right.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.lower_right, source_frame);
        _ = self.lower_middle.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.lower_middle, source_frame);
        _ = self.center.in_place_crop_and_nearest_neighbor_resize_to_self(&self.segment_corner_points.center, source_frame);
        Ok(())
    }
    

    
    
}

/// Stores the corner points for each segment of a segmented vision frame
///
/// This structure defines the precise pixel coordinates for each of the nine segments
/// of a segmented vision frame.
#[derive(PartialEq)]
#[derive(Debug)]
struct SegmentedCornerPoints {
    /// Corner points for the lower-left segment
    lower_left: CornerPoints,
    /// Corner points for the middle-left segment
    middle_left: CornerPoints,
    /// Corner points for the upper-left segment
    upper_left: CornerPoints,
    /// Corner points for the upper-middle segment
    upper_middle: CornerPoints,
    /// Corner points for the upper-right segment
    upper_right: CornerPoints,
    /// Corner points for the middle-right segment
    middle_right: CornerPoints,
    /// Corner points for the lower-right segment
    lower_right: CornerPoints,
    /// Corner points for the lower-middle segment
    lower_middle: CornerPoints,
    /// Corner points for the center segment
    center: CornerPoints,
}

impl SegmentedCornerPoints {
    /// Creates a new SegmentedCornerPoints from source resolution and center corner points
    ///
    /// This method calculates the corner points for all nine segments based on the
    /// center region and the overall resolution.
    ///
    /// # Arguments
    ///
    /// * `source_full_resolution` - Total resolution (width, height) of the source frame
    /// * `center_corner_points` - Corner points defining the center region
    ///
    /// # Returns
    ///
    /// * `Result<SegmentedCornerPoints, &'static str>` - Created instance or error message
    ///
    /// # Errors
    ///
    /// Returns an error if the center corner points don't fit within the source resolution
    pub fn from_source_and_center_corner_points(source_full_resolution: (usize, usize), center_corner_points: CornerPoints) -> Result<SegmentedCornerPoints, &'static str> {
        if !center_corner_points.does_fit_in_frame_of_resolution(source_full_resolution){
            return Err("The corner points cannot exceed the range of the full resolution!");
        }
        Ok(SegmentedCornerPoints{
            lower_left: CornerPoints::new((0,0), center_corner_points.lower_left).unwrap(),
            middle_left: CornerPoints::new((0,center_corner_points.lower_left.1), center_corner_points.upper_left()).unwrap(),
            upper_left: CornerPoints::new((0,center_corner_points.upper_left().1), (center_corner_points.upper_left().0, source_full_resolution.1)).unwrap(),
            upper_middle: CornerPoints::new(center_corner_points.upper_left(), (center_corner_points.upper_right.0, source_full_resolution.1)).unwrap(),
            upper_right: CornerPoints::new(center_corner_points.upper_right, source_full_resolution).unwrap(),
            middle_right: CornerPoints::new(center_corner_points.lower_right(), (source_full_resolution.0, center_corner_points.upper_right.1)).unwrap(),
            lower_right: CornerPoints::new((center_corner_points.lower_right().0, 0), (source_full_resolution.0, center_corner_points.lower_right().1)).unwrap(),
            lower_middle: CornerPoints::new((center_corner_points.lower_left.0, 0), center_corner_points.lower_right()).unwrap(),
            center: center_corner_points
        })
    }
}