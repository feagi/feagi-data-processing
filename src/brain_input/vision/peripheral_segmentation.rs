use super::ImageFrame;
use crate::error::DataProcessingError;
use super::descriptors::*;
use crate::cortical_data::CorticalID;
use crate::neuron_data::{CorticalMappedNeuronData, NeuronXYCPArrays};

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
        let inner_corners = center_properties.calculate_pixel_coordinates_of_center_corners(source_frame_width_height)?;
        let segment_corner_points = SegmentedCornerPoints::from_source_and_center_corner_points(source_frame_width_height, inner_corners)?;
        

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
    
    pub fn export_as_new_cortical_mapped_neuron_data(&mut self, camera_index: u8) -> Result<CorticalMappedNeuronData, DataProcessingError> {

        let ordered_refs: [&mut ImageFrame; 9] = [&mut self.center, &mut self.lower_left, &mut self.middle_left,
            &mut self.upper_left, &mut self.upper_middle, &mut self.upper_right, &mut self.middle_right, &mut self.lower_right,
            &mut self.lower_middle];
        
        let mut cortical_ids: [CorticalID; 9] = [CorticalID::from_str("iv00_C")?,
            CorticalID::from_str("iv00BL")?, CorticalID::from_str("iv00ML")?,
            CorticalID::from_str("iv00TL")?, CorticalID::from_str("iv00TM")?,
            CorticalID::from_str("iv00TR")?, CorticalID::from_str("iv00MR")?,
            CorticalID::from_str("iv00BR")?, CorticalID::from_str("iv00BM")?]; // same order as other struct members

        if ordered_refs[0].get_color_channel_count() > 1 {
            // Ensure we aren't using grays scale cortical area ID if we are doing things in color
            cortical_ids[0] = CorticalID::from_str("iv00CC")?
        }
        
        // TODO user camera index
        /*
        let replacement_chars = self.u8_to_hex_chars(camera_index);
        for cortical_id_string in cortical_ids_strings.iter_mut(){
            cortical_id_string.replace_range(2..4, &format!("{}{}", replacement_chars.0, replacement_chars.1));
        };
        
         */

        let mut output: CorticalMappedNeuronData = CorticalMappedNeuronData::new();
        
        for index in 0..9 {
            let max_neurons = ordered_refs[index].get_max_capacity_neuron_count();
            let mut data: NeuronXYCPArrays = NeuronXYCPArrays::new(max_neurons)?;
            ordered_refs[index].write_thresholded_xyzp_neuron_arrays(10.0, &mut data);
            output.insert(cortical_ids[index].clone(), data);
        }
        
        Ok(output)
    }
    
    pub fn inplace_export_cortical_mapped_neuron_data(&mut self, ordered_cortical_IDs: [CorticalID; 9], all_mapped_neuron_data: &mut CorticalMappedNeuronData) -> Result<(), DataProcessingError> {
        let ordered_refs: [&mut ImageFrame; 9] = [&mut self.center, &mut self.lower_left, &mut self.middle_left,
            &mut self.upper_left, &mut self.upper_middle, &mut self.upper_right, &mut self.middle_right, &mut self.lower_right,
            &mut self.lower_middle];
        
        let id_counter: usize = 0;
        for index in 0..9 {
            let cortical_id = &ordered_cortical_IDs[index];
            let mapped_neuron_data = all_mapped_neuron_data.get_mut(cortical_id);
            match mapped_neuron_data { 
                None => {
                    return Err(DataProcessingError::InternalError("Unable to find cortical area to unwrap!".into())); // TODO specific error?
                }
                Some(mapped_data) => {
                    ordered_refs[index].write_thresholded_xyzp_neuron_arrays(10.0, mapped_data)?;
                }
            }
        }
        Ok(())
    }
    
    
    
    fn u8_to_hex_chars(& self, n: u8) -> (char, char) { // TODO this should be moved elsewhere // TODO moving this to cortical ID makes sense
        const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
        let high = HEX_CHARS[(n >> 4) as usize] as char;
        let low = HEX_CHARS[(n & 0x0F) as usize] as char;
        (high, low)
    }


    
}


// region internal helpers


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
            lower_left: CornerPoints::new_from_row_major((source_width_height.1, 0), center_corner_points.lower_left_row_major())?,
            middle_left: CornerPoints::new_from_row_major((center_corner_points.lower_left_row_major().0, 0), center_corner_points.upper_left_row_major())?,
            upper_left: CornerPoints::new_from_row_major((center_corner_points.upper_right_row_major().0, 0), (0, center_corner_points.lower_left_row_major().1))?,
            upper_middle: CornerPoints::new_from_row_major(center_corner_points.upper_left_row_major(), (0, center_corner_points.upper_right_row_major().1))?,
            upper_right: CornerPoints::new_from_row_major(center_corner_points.upper_right_row_major(), (0, source_width_height.0))?,
            middle_right: CornerPoints::new_from_row_major(center_corner_points.lower_right_row_major(), (center_corner_points.upper_right_row_major().0, source_width_height.0))?,
            lower_right: CornerPoints::new_from_row_major((source_width_height.1, center_corner_points.upper_right_row_major().1), (center_corner_points.lower_left_row_major().1, source_width_height.0))?,
            lower_middle: CornerPoints::new_from_row_major((source_width_height.1, center_corner_points.lower_left_row_major().1), center_corner_points.lower_right_row_major())?,
            center: center_corner_points
        })
    }
}

// endregion