//! Segmented vision frame processing for FEAGI peripheral vision simulation.
//! 
//! This module provides the `SegmentedVisionFrame` struct which divides an input image
//! into nine segments with different resolutions to simulate peripheral vision. The center
//! segment typically has higher resolution while peripheral segments have lower resolution,
//! mimicking how human vision works with high acuity in the center and lower acuity in
//! the periphery.

use super::image_frame::ImageFrame;
use crate::error::{FeagiDataProcessingError, IODataError};
use super::descriptors::*;
use crate::genomic_structures::{CorticalGroupingIndex, CorticalID};
use crate::neuron_data::{CorticalMappedXYZPNeuronData, NeuronXYZPArrays};


/// A frame divided into nine segments with different resolutions for peripheral vision simulation.
///
/// This structure represents a segmented view of a source frame, dividing it into nine regions:
/// - **Center**: High-resolution central region (foveal vision)
/// - **Eight peripheral segments**: Lower-resolution surrounding regions (peripheral vision)
/// 
/// The segmentation pattern follows this layout:
/// ```text
/// ┌─────────┬─────────┬─────────┐
/// │ upper_  │ upper_  │ upper_  │
/// │ left    │ middle  │ right   │
/// ├─────────┼─────────┼─────────┤
/// │ middle_ │ center  │ middle_ │
/// │ left    │         │ right   │
/// ├─────────┼─────────┼─────────┤
/// │ lower_  │ lower_  │ lower_  │
/// │ left    │ middle  │ right   │
/// └─────────┴─────────┴─────────┘
/// ```
/// 
/// This design allows FEAGI to process visual information with varying levels of detail,
/// concentrating computational resources in the center of attention while maintaining
/// awareness of the broader visual field.
#[derive(Clone)]  // TODO Shouldnt this be called Segmented Image Frame?
pub struct SegmentedImageFrame {
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
    /// Resolution of the original source frame that was loaded into this
    previous_imported_internal_yx_resolution: (usize, usize), // All imported frames need to match this
    /// The cropping points to use for the source, cached, assuming the source resolution is the same
    previous_cropping_points_for_source_from_segment: Option<SegmentedVisionFrameSourceCroppingPointGrouping>
}

impl SegmentedImageFrame {

    //region common constructors
    
    /// Creates a new SegmentedVisionFrame with specified resolutions and color properties.
    /// 
    /// This constructor initializes all nine segments with their respective resolutions
    /// and the same color format and color space. Each segment is created as an empty
    /// ImageFrame ready to receive cropped and resized data from source images.
    /// 
    /// # Arguments
    /// 
    /// * `segment_resolutions` - The target resolutions for each of the nine segments
    /// * `segment_color_channels` - The color channel format (GrayScale, RG, RGB, or RGBA)
    /// * `segment_color_space` - The color space (Linear or Gamma)
    /// * `input_frames_source_width_height` - The expected resolution of source frames (width, height)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(SegmentedVisionFrame) if all segments were created successfully
    /// - Err(DataProcessingError) if any segment creation fails
    pub fn new(segment_resolutions: &SegmentedVisionTargetResolutions, segment_color_channels: &ChannelFormat,
    segment_color_space: &ColorSpace, input_frames_source_width_height: (usize, usize)) -> Result<SegmentedImageFrame, FeagiDataProcessingError> {
        Ok(SegmentedImageFrame {
            lower_left: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.lower_left),
            middle_left: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.middle_left),
            upper_left: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.upper_left),
            upper_middle: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.upper_middle),
            upper_right: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.upper_right),
            middle_right: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.middle_right),
            lower_right: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.lower_right),
            lower_middle: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.lower_middle),
            center: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.center),
            previous_imported_internal_yx_resolution: (input_frames_source_width_height.1, input_frames_source_width_height.0),
            previous_cropping_points_for_source_from_segment: None,
        })
    }
    
    //endregion
    
    //region get properties
    
    /// Returns the color space used by all segments in this frame.
    /// 
    /// Since all segments share the same color space, this method returns
    /// a reference to the color space from any segment (using upper_left as representative).
    /// 
    /// # Returns
    /// 
    /// A reference to the ColorSpace enum value.
    pub fn get_color_space(&self) -> &ColorSpace {
        self.upper_left.get_color_space()
    }

    /// Returns the color channel format used by all segments in this frame.
    /// 
    /// Since all segments share the same channel format, this method returns
    /// a reference to the channel format from any segment (using upper_left as representative).
    /// 
    /// # Returns
    /// 
    /// A reference to the ChannelFormat enum value.
    pub fn get_color_channels(&self) -> &ChannelFormat {
        self.upper_left.get_channel_format()
    }
    
    //endregion
    
    //region Loading in new data
    
    /// Updates all nine segments with data from a source frame.
    /// 
    /// This method takes a source ImageFrame and divides it into nine segments according
    /// to the center properties. Each segment is cropped from the appropriate region of
    /// the source frame and resized to match the target resolution for that segment.
    /// 
    /// The method caches cropping points for efficiency - if the same source resolution
    /// is used repeatedly, the cropping calculations are only performed once.
    /// 
    /// # Arguments
    /// 
    /// * `source_frame` - The source ImageFrame to segment
    /// * `center_properties` - Properties defining how to position and size the center region
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if all segments were updated successfully
    /// - Err(DataProcessingError) if the source frame is incompatible or processing fails
    /// 
    /// # Errors
    /// 
    /// This method will return an error if:
    /// - The source frame has different color channels than expected
    /// - The source frame has a different color space than expected
    /// - The source frame has a different resolution than expected
    /// - Any of the cropping or resizing operations fail
    pub fn update_segments(&mut self, source_frame: &ImageFrame, 
                           center_properties: SegmentedVisionCenterProperties)
        -> Result<(), FeagiDataProcessingError> {
        if source_frame.get_channel_format() != self.get_color_channels(){
            return Err(IODataError::InvalidParameters("Input Image frame does not have matching color channel count!".into()).into());
        }
        if source_frame.get_color_space() != self.get_color_space() {
            return Err(IODataError::InvalidParameters("Input Image frame does not have matching color space!".into()).into());
        }
        if source_frame.get_internal_resolution() != self.previous_imported_internal_yx_resolution {
            return Err(IODataError::InvalidParameters("Input Image frame does not have matching resolution!".into()).into());
        }
        
        if self.previous_cropping_points_for_source_from_segment.is_none() {
            
            // We either have no corner points for the cropping sources defined, or they are no longer
            // valid, we need to update them
            self.previous_cropping_points_for_source_from_segment = Some(
                center_properties.calculate_source_corner_points_for_segemented_video_frame(source_frame.get_cartesian_width_height())?);
        }
        
        let cropping_points= self.previous_cropping_points_for_source_from_segment.unwrap(); // We know this exists by now
        
        self.lower_left.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.lower_left, source_frame)?;
        self.middle_left.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.middle_left, source_frame)?;
        self.upper_left.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.upper_left, source_frame)?;
        self.upper_middle.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.upper_middle, source_frame)?;
        self.upper_right.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.upper_right, source_frame)?;
        self.middle_right.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.middle_right, source_frame)?;
        self.lower_right.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.lower_right, source_frame)?;
        self.lower_middle.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.lower_middle, source_frame)?;
        self.center.in_place_crop_and_nearest_neighbor_resize(
            &cropping_points.center, source_frame)?;
        
        Ok(())
        
    }
    
    //endregion
    
    //region neuron export
    
    /// Exports all segments as a new cortical-mapped neuron data structure.
    /// 
    /// This method converts each of the nine segments into neuron data and maps them
    /// to their corresponding cortical areas. Each segment is processed with a threshold
    /// to determine which pixels become neurons, and the resulting data is organized
    /// by cortical ID.
    /// 
    /// The cortical IDs follow a standard naming convention:
    /// - Center: "iv00_C" (grayscale) or "iv00CC" (color)
    /// - Peripheral segments: "iv00BL", "iv00ML", "iv00TL", "iv00TM", "iv00TR", "iv00MR", "iv00BR", "iv00BM"
    /// 
    /// # Arguments
    /// 
    /// * `_camera_index` - The camera index (currently unused but reserved for future multi-camera support)
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(CorticalMappedNeuronData) with neuron data for all nine segments
    /// - Err(DataProcessingError) if any conversion fails
    pub fn export_as_new_cortical_mapped_neuron_data(&mut self, camera_index: CorticalGroupingIndex) -> Result<CorticalMappedXYZPNeuronData, FeagiDataProcessingError> {

        let ordered_refs: [&mut ImageFrame; 9] = self.get_ordered_image_frame_references();
        
        let cortical_ids: [CorticalID; 9] = CorticalID::create_ordered_cortical_areas_for_segmented_vision(camera_index, ordered_refs[0].get_color_channel_count() == 1);
        
        let mut output: CorticalMappedXYZPNeuronData = CorticalMappedXYZPNeuronData::new();
        
        for index in 0..9 {
            let max_neurons = ordered_refs[index].get_max_capacity_neuron_count();
            let mut data: NeuronXYZPArrays = NeuronXYZPArrays::new(max_neurons)?;
            ordered_refs[index].write_thresholded_xyzp_neuron_arrays(10.0, &mut data)?;
            output.insert(cortical_ids[index].clone(), data);
        }
        
        Ok(output)
    }
    
    /// Exports neuron data from all segments into an existing cortical-mapped data structure.
    /// 
    /// This method is similar to `export_as_new_cortical_mapped_neuron_data` but writes
    /// the neuron data into pre-existing NeuronXYCPArrays structures. This is more efficient
    /// when the cortical data structure is being reused across multiple frames.
    /// 
    /// # Arguments
    /// 
    /// * `ordered_cortical_ids` - An array of 9 cortical IDs in the expected order:
    ///   [center, lower_left, middle_left, upper_left, upper_middle, upper_right, middle_right, lower_right, lower_middle]
    /// * `all_mapped_neuron_data` - The existing cortical-mapped data structure to write into
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if all segments were exported successfully
    /// - Err(DataProcessingError) if any cortical ID is not found or conversion fails
    pub fn inplace_export_cortical_mapped_neuron_data(&mut self, ordered_cortical_ids: &[CorticalID; 9], all_mapped_neuron_data: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        let ordered_refs: [&mut ImageFrame; 9] = self.get_ordered_image_frame_references();
        
        for index in 0..9 {
            let cortical_id = &ordered_cortical_ids[index];
            let mapped_neuron_data = all_mapped_neuron_data.borrow_mut(cortical_id);
            match mapped_neuron_data { 
                None => {
                    return Err(FeagiDataProcessingError::InternalError("Unable to find cortical area to unwrap!".into())); // TODO specific error?
                }
                Some(mapped_data) => {
                    ordered_refs[index].write_thresholded_xyzp_neuron_arrays(10.0, mapped_data)?;
                }
            }
        }
        Ok(())
    }
    
    //endregion
    
    //region internal functions
    
    /// Returns mutable references to all nine image frames in the standard order.
    /// 
    /// This internal helper method provides ordered access to the image frame segments
    /// for operations that need to process all segments uniformly.
    /// 
    /// # Returns
    /// 
    /// An array of mutable references to the nine ImageFrame segments in the order:
    /// [center, lower_left, middle_left, upper_left, upper_middle, upper_right, middle_right, lower_right, lower_middle]
    fn get_ordered_image_frame_references(&mut self) -> [&mut ImageFrame; 9] {
        [&mut self.center, &mut self.lower_left, &mut self.middle_left,
            &mut self.upper_left, &mut self.upper_middle, &mut self.upper_right, &mut self.middle_right, &mut self.lower_right,
            &mut self.lower_middle]
    }
    
    //endregion


    
}

