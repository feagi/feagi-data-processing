use super::single_frame::ImageFrame;
use crate::Error::DataProcessingError;
use super::single_frame_processing::*;
use std::cmp;
use ndarray::Array3;

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
    // /// Corner points defining the boundaries of each segment
    //segment_corner_points: SegmentedCornerPoints,
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
    pub fn new(source_frame: &ImageFrame, center_properties: &SegmentedVisionCenterProperties, segment_resolutions: &SegmentedVisionTargetResolutions) -> Result<SegmentedVisionFrame, DataProcessingError> {
        let source_frame_width_height: (usize, usize) = source_frame.get_internal_resolution();
        //let inner_corners = center_properties.calculate_pixel_coordinates_of_center_corners(source_frame_width_height)?;
        //let segment_corner_points = SegmentedCornerPoints::from_source_and_center_corner_points(source_frame_width_height, inner_corners)?;
        
        /*
        // For all the following, we know the crops are safe
        Ok(SegmentedVisionFrame{
            lower_left: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.lower_left, &segment_resolutions.lower_left)?,
            middle_left: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.middle_left, &segment_resolutions.middle_left)?,
            upper_left: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.upper_left, &segment_resolutions.upper_left)?,
            upper_middle: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.upper_middle, &segment_resolutions.upper_middle)?,
            upper_right: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.upper_right, &segment_resolutions.upper_right)?,
            middle_right: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.middle_right, &segment_resolutions.middle_right)?,
            lower_right: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.lower_right, &segment_resolutions.lower_right)?,
            lower_middle: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.lower_middle, &segment_resolutions.lower_middle)?,
            center: ImageFrame::create_from_source_frame_crop_and_resize(source_frame, &segment_corner_points.center, &segment_resolutions.center)?,
            original_source_resolution: source_frame_width_height,
            segment_corner_points
        })
        
         */
        
        Ok(SegmentedVisionFrame{
            lower_left: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            middle_left: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            upper_left: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            upper_middle: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            upper_right: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            middle_right: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            lower_right: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            lower_middle: ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(1, 1)),
            center: source_frame.clone(),
            original_source_resolution: source_frame_width_height,
        })
        
    }

    /*
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
    */

    /*
    pub fn export_as_neuron_potential_data(& self, camera_index: u8) -> Result<CorticalMappedNeuronPotentialCollectionXYZ, &'static str> {
        let index_ID = self.u8_to_hex_chars(camera_index);

    }
     */

    /// Exports the segmented vision frame as a byte array containing neuron potential data. See FEAGI byte Structure 11 for more details.
    ///
    /// This function converts the segmented vision frame into a binary form. The output includes
    /// headers and data for all nine segments (center and peripheral regions), with each segment's
    /// data containing XYZ coordinates and potential values.
    ///
    /// # Arguments
    ///
    /// * `camera_index` - An 8-bit identifier for the camera source (0-255)
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, &'static str>` - A byte vector containing the formatted data or an error message
    ///
    /// # Format
    ///
    /// The output byte array follows this structure:
    /// - Global header (2 bytes): Structure ID and version (11, 1)
    /// - Cortical count header (2 bytes): Number of cortical areas (u16)
    /// - Per-cortical headers (14 bytes each): ID, start index, and length for each segment
    /// - Data section: XYZP (X,Y,Z coordinates and potential) values for each segment
    /// ```
    pub fn direct_export_as_byte_neuron_potential_categorical_xyz(&self, camera_index: u8, pixel_abs_threshold: f32) -> Result<Vec<u8>, DataProcessingError> {

        const BYTE_STRUCT_ID: u8 = 11;
        const BYTE_STRUCT_VERSION: u8 = 1;
        const CORTICAL_AREA_COUNT: u16 = 9;
        const GLOBAL_HEADER_SIZE: usize = 2;
        const CORTICAL_COUNT_HEADER_SIZE: usize = 2;
        const PER_CORTICAL_HEADER_DESCRIPTOR_SIZE: usize = 14;
        const PER_NEURON_XYZP_SIZE: usize = 16;

        // Calculate prerequisite info

        let ordered_refs: [&ImageFrame; 9] = [&self.center, &self.lower_left, &self.middle_left,
        &self.upper_left, &self.upper_middle, &self.upper_right, &self.middle_right, &self.lower_right,
        &self.lower_middle];

        let mut cortical_ids: [String; 9] = [String::from("iv00_C"),
            String::from("iv00BL"), String::from("iv00ML"), String::from("iv00TL"),
            String::from("iv00TM"), String::from("iv00TR"), String::from("iv00MR"),
            String::from("iv00BR"), String::from("iv00BM")]; // same order as other struct members
        if self.center.get_color_channel_count() > 1 {
            // Ensure we aren't using grays scale cortical area ID if we are doing things in color
            cortical_ids[0] = String::from("iv00CC")
        }
        let replacement_chars = self.u8_to_hex_chars(camera_index);
        for cortical_id_string in cortical_ids.iter_mut(){
            cortical_id_string.replace_range(2..4, &format!("{}{}", replacement_chars.0, replacement_chars.1));
        }
        let cortical_data_per_segment: [Vec<u8>; 9] = [
            self.center.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.lower_left.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.middle_left.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.upper_left.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.upper_middle.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.upper_right.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.middle_right.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.lower_right.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
            self.lower_middle.as_thresholded_xyzp_byte_data(pixel_abs_threshold)?,
        ];
        
        
        
        
        
        
        let number_bytes_per_segment: [usize; 9] = ordered_refs.map(|s| s.get_number_of_bytes_needed_to_hold_xyzp_uncompressed());
        
        let byte_array_length: usize = GLOBAL_HEADER_SIZE + CORTICAL_COUNT_HEADER_SIZE +
            (CORTICAL_AREA_COUNT as usize * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE) +
            number_bytes_per_segment.iter().sum::<usize>();
        
        // Create Byte Array, and fill in header
        
        let mut output: Vec<u8> = vec![0; byte_array_length];
        output[0] = BYTE_STRUCT_ID;
        output[1] = BYTE_STRUCT_VERSION;
        
        let count_bytes: [u8; 2] = CORTICAL_AREA_COUNT.to_le_bytes();
        output[2..4].copy_from_slice(&count_bytes);
        
        let mut header_write_index: usize = 4;
        let mut data_write_index: u32 = 4 + (CORTICAL_AREA_COUNT as u32 * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE as u32);
        
        for cortical_index in 0..CORTICAL_AREA_COUNT as usize {
            let cortical_id_bytes = (&cortical_ids[cortical_index]).as_bytes(); // We know this to be ascii
            let reading_start_index: u32 = data_write_index;
            let reading_start_index_bytes: [u8; 4] = reading_start_index.to_le_bytes();
            let reading_length: u32 = number_bytes_per_segment[cortical_index] as u32; // TODO divide by 4
            let reading_length_bytes: [u8; 4] = reading_length.to_le_bytes();

            output[header_write_index..header_write_index + 6].copy_from_slice(cortical_id_bytes);
            output[header_write_index + 6.. header_write_index + 10].copy_from_slice(&reading_start_index_bytes);
            output[header_write_index + 10.. header_write_index + 14].copy_from_slice(&reading_length_bytes);
            
            header_write_index += PER_CORTICAL_HEADER_DESCRIPTOR_SIZE;
            data_write_index += reading_length;
        };
        
        // fill in data
        let data_write_index: usize = 4 + (CORTICAL_AREA_COUNT as usize * PER_CORTICAL_HEADER_DESCRIPTOR_SIZE);
        for cortical_index in 0..CORTICAL_AREA_COUNT as usize {
            let data_length: usize = ordered_refs[cortical_index].get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
            let relevant_slice: &mut [u8] = &mut output[data_write_index..data_write_index + data_length];
            let _ = ordered_refs[cortical_index].to_bytes_in_place(relevant_slice)?;
        };
        Ok(output)
    }

    pub fn get_center_image_frame(&self) -> &ImageFrame {
        &self.center
    }

    fn u8_to_hex_chars(& self, n: u8) -> (char, char) { // TODO this should be moved elsewhere
        const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
        let high = HEX_CHARS[(n >> 4) as usize] as char;
        let low = HEX_CHARS[(n & 0x0F) as usize] as char;
        (high, low)
    }
    
}


// region helpers

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
    pub fn new_row_major_where_origin_top_left(center_coordinates_normalized_yx_rm_tl: (f32, f32), center_size_yx_normalized: (f32, f32)) -> Result<SegmentedVisionCenterProperties, DataProcessingError> {
        if center_coordinates_normalized_yx_rm_tl.0 < 0.0 || center_coordinates_normalized_yx_rm_tl.1 < 0.0 || center_coordinates_normalized_yx_rm_tl.0 > 1.0 || center_coordinates_normalized_yx_rm_tl.1 > 1.0 {
            return Err(DataProcessingError::InvalidInputBounds("Coordinates are to be normalized and must be between 0 and 1!".into()))
        }
        if center_size_yx_normalized.0 < 0.0 || center_size_yx_normalized.1 < 0.0 || center_size_yx_normalized.0 > 1.0 || center_size_yx_normalized.1 > 1.0 {
            return Err(DataProcessingError::InvalidInputBounds("Central vision size is to be normalized and must be between 0 and 1!".into()))
        }
        Ok(SegmentedVisionCenterProperties {
            center_coordinates_normalized_yx: center_coordinates_normalized_yx_rm_tl,
            center_size_normalized_yx: center_size_yx_normalized,
        })
    }

    pub fn cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_yx: (f32, f32), center_size_normalized_yx: (f32, f32)) -> Result<SegmentedVisionCenterProperties, DataProcessingError> {
        SegmentedVisionCenterProperties::new_row_major_where_origin_top_left(
            (1.0 - center_coordinates_normalized_cartesian_yx.0, center_coordinates_normalized_cartesian_yx.1),
            center_size_normalized_yx)
    }

    pub fn create_default_centered() -> SegmentedVisionCenterProperties {
        SegmentedVisionCenterProperties::new_row_major_where_origin_top_left((0.5, 0.5), (0.5, 0.5)).unwrap()
    }

    /// Returns an instance of CornerPoints that defines the region of the center region by pixel index
    ///
    /// # Arguments
    ///
    /// * `source_frame_width_height` - The total resolution (width, height) of the source frame in pixels
    ///
    /// # Returns
    ///
    /// * `Result<CornerPoints, &'static str>` - Corner points that define the center region or an error message
    ///
    /// # Errors
    ///
    /// Returns an error if the source width or height is less than 3 pixels
    pub fn calculate_pixel_coordinates_of_center_corners(&self, source_frame_width_height: (usize, usize)) -> Result<CornerPoints, DataProcessingError> {
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

        let corner_points: CornerPoints = CornerPoints::new_from_row_major_where_origin_top_left(
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
        return SegmentedVisionTargetResolutions::new(peripheral_width_height, peripheral_width_height,
                                                     peripheral_width_height, peripheral_width_height,
                                                     peripheral_width_height, peripheral_width_height,
                                                     peripheral_width_height, peripheral_width_height, center_width_height);
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
    /// This method calculates the corner points source for all nine segments based on the
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
    pub fn from_source_and_center_corner_points(source_width_height: (usize, usize), center_corner_points: CornerPoints) -> Result<SegmentedCornerPoints, DataProcessingError> {
        if !center_corner_points.does_fit_in_frame_of_width_height(source_width_height){
            return Err(DataProcessingError::InvalidInputBounds("The corner points cannot exceed the range of the full resolution!".into()));
        }
        Ok(SegmentedCornerPoints{
            lower_left: CornerPoints::new_from_row_major_where_origin_top_left((source_width_height.1,0), center_corner_points.lower_left_row_major())?,
            middle_left: CornerPoints::new_from_row_major_where_origin_top_left((center_corner_points.lower_left_row_major().0, 0), center_corner_points.upper_left_row_major())?,
            upper_left: CornerPoints::new_from_row_major_where_origin_top_left((center_corner_points.upper_right_row_major().0,0), (0, center_corner_points.lower_left_row_major().1))?,
            upper_middle: CornerPoints::new_from_row_major_where_origin_top_left(center_corner_points.upper_left_row_major(), (0, center_corner_points.upper_right_row_major().1))?,
            upper_right: CornerPoints::new_from_row_major_where_origin_top_left(center_corner_points.upper_right_row_major(), (0, source_width_height.0))?,
            middle_right: CornerPoints::new_from_row_major_where_origin_top_left(center_corner_points.lower_right_row_major(), (center_corner_points.upper_right_row_major().0, source_width_height.0))?,
            lower_right: CornerPoints::new_from_row_major_where_origin_top_left((source_width_height.1, center_corner_points.upper_right_row_major().1), (center_corner_points.lower_left_row_major().1, source_width_height.0))?,
            lower_middle: CornerPoints::new_from_row_major_where_origin_top_left((source_width_height.1, center_corner_points.lower_left_row_major().1), center_corner_points.lower_right_row_major())?,
            center: center_corner_points
        })
    }
}

// endregion