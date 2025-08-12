//! Segmented vision frame processing for FEAGI peripheral vision simulation.
//! 
//! This module provides the `SegmentedVisionFrame` struct which divides an input image
//! into nine segments with different resolutions to simulate peripheral vision. The center
//! segment typically has higher resolution while peripheral segments have lower resolution,
//! mimicking how human vision works with high acuity in the center and lower acuity in
//! the periphery.

use ndarray::Array3;
use super::image_frame::ImageFrame;
use crate::error::{FeagiDataProcessingError};
use super::descriptors::*;
use crate::genomic_structures::{CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex};
use crate::io_data::image::descriptors::ImageFrameProperties;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPArrays};


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
#[derive(Clone, Debug)]
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

impl std::fmt::Display for SegmentedImageFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SegmentedImageFrame()")
    }
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
    pub fn new(segment_resolutions: &SegmentedFrameTargetResolutions, segment_color_channels: &ChannelLayout,
               segment_color_space: &ColorSpace, input_frames_source_width_height: (usize, usize)) -> Result<SegmentedImageFrame, FeagiDataProcessingError> {
        Ok(SegmentedImageFrame {
            lower_left: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.lower_left)?,
            middle_left: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.middle_left)?,
            upper_left: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.upper_left)?,
            upper_middle: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.upper_middle)?,
            upper_right: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.upper_right)?,
            middle_right: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.middle_right)?,
            lower_right: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.lower_right)?,
            lower_middle: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.lower_middle)?,
            center: ImageFrame::new(&segment_color_channels, &segment_color_space, &segment_resolutions.center)?,
            previous_imported_internal_yx_resolution: (input_frames_source_width_height.1, input_frames_source_width_height.0),
            previous_cropping_points_for_source_from_segment: None,
        })
    }
    
    //endregion
    
    //region get properties
    
    /// Returns the properties of all nine image frame segments.
    ///
    /// Provides the image properties (resolution, color space, channel layout) for each
    /// of the nine segments in the standard cortical ordering: center, lower_left, 
    /// middle_left, upper_left, upper_middle, upper_right, middle_right, lower_right, 
    /// lower_middle.
    ///
    /// # Returns
    ///
    /// An array of 9 ImageFrameProperties, one for each segment in cortical order.
    pub fn get_image_frame_properties(&self) -> [ImageFrameProperties; 9] {
        // return in same order as cortical IDs
        [
            self.lower_left.get_image_frame_properties(),
            self.lower_middle.get_image_frame_properties(),
            self.lower_right.get_image_frame_properties(),
            self.middle_left.get_image_frame_properties(),
            self.center.get_image_frame_properties(),
            self.middle_right.get_image_frame_properties(),
            self.upper_left.get_image_frame_properties(),
            self.upper_middle.get_image_frame_properties(),
            self.upper_right.get_image_frame_properties(),
        ]
    }
    
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
    
    /// Returns the channel layout of the center segment.
    ///
    /// # Returns
    ///
    /// A reference to the ChannelLayout enum value for the center segment.
    pub fn get_center_channel_layout(&self) -> &ChannelLayout {
        self.center.get_channel_layout()
    }
    
    /// Returns the channel layout of the peripheral segments.
    ///
    /// All peripheral segments (non-center) are expected to have the same channel layout.
    /// This method returns the layout from the lower_left segment as representative.
    ///
    /// # Returns
    ///
    /// A reference to the ChannelLayout enum value for the peripheral segments.
    pub fn get_peripheral_channel_layout(&self) -> &ChannelLayout {
        self.lower_left.get_channel_layout() // All peripherals should be the same
    }
    
    /// Returns references to the internal pixel data arrays for all nine segments.
    ///
    /// Provides direct access to the underlying 3D arrays containing pixel data
    /// for each segment. The arrays are returned in the standard cortical ordering.
    ///
    /// # Returns
    ///
    /// An array of 9 references to Array3<f32>, one for each segment in cortical order.
    pub fn get_image_internal_data(&self) -> [&Array3<f32>; 9] {
        // return in same order as cortical IDs
        [
            self.lower_left.get_internal_data(),
            self.lower_middle.get_internal_data(),
            self.lower_right.get_internal_data(),
            self.middle_left.get_internal_data(),
            self.center.get_internal_data(),
            self.middle_right.get_internal_data(),
            self.upper_left.get_internal_data(),
            self.upper_middle.get_internal_data(),
            self.upper_right.get_internal_data(),
        ]
    }
    
    pub(crate) fn get_image_internal_data_mut(&mut self) -> [&mut Array3<f32>; 9] {
        // return in same order as cortical IDs
        [
            self.lower_left.get_internal_data_mut(),
            self.lower_middle.get_internal_data_mut(),
            self.lower_right.get_internal_data_mut(),
            self.middle_left.get_internal_data_mut(),
            self.center.get_internal_data_mut(),
            self.middle_right.get_internal_data_mut(),
            self.upper_left.get_internal_data_mut(),
            self.upper_middle.get_internal_data_mut(),
            self.upper_right.get_internal_data_mut(),
        ]
    }
    
    //endregion
    
    //region neuron export
    /// Exports all segments as new cortical mapped neuron data.
    ///
    /// Converts the pixel data from all nine image segments into neuron data format
    /// suitable for FEAGI processing. Creates a new CorticalMappedXYZPNeuronData container
    /// with the appropriate cortical IDs and spatial mappings.
    ///
    /// # Arguments
    ///
    /// * `camera_index` - The cortical grouping index for the camera/vision system
    /// * `channel_index` - The channel index within the cortical IO system
    ///
    /// # Returns
    ///
    /// * `Ok(CorticalMappedXYZPNeuronData)` - Successfully created neuron data
    /// * `Err(FeagiDataProcessingError)` - If the conversion fails
    pub fn export_as_new_cortical_mapped_neuron_data(&mut self, camera_index: CorticalGroupingIndex, channel_index: CorticalIOChannelIndex) -> Result<CorticalMappedXYZPNeuronData, FeagiDataProcessingError> {

        let ordered_refs: [&mut ImageFrame; 9] = self.get_ordered_image_frame_references();
        
        let cortical_ids: [CorticalID; 9] = CorticalID::create_ordered_cortical_areas_for_segmented_vision(camera_index);
        
        let mut output: CorticalMappedXYZPNeuronData = CorticalMappedXYZPNeuronData::new();
        
        for index in 0..9 {
            let max_neurons = ordered_refs[index].get_max_capacity_neuron_count();
            let mut data: NeuronXYZPArrays = NeuronXYZPArrays::with_capacity(max_neurons);
            ordered_refs[index].write_xyzp_neuron_arrays(&mut data, channel_index)?;
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
    pub fn inplace_export_cortical_mapped_neuron_data(&mut self, ordered_cortical_ids: &[CorticalID; 9], all_mapped_neuron_data: &mut CorticalMappedXYZPNeuronData, channel_index: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {
        let ordered_refs: [&mut ImageFrame; 9] = self.get_ordered_image_frame_references();
        
        for index in 0..9 {
            let cortical_id = &ordered_cortical_ids[index];
            let mapped_neuron_data = all_mapped_neuron_data.get_neurons_of_mut(cortical_id);
            match mapped_neuron_data { 
                None => {
                    return Err(FeagiDataProcessingError::InternalError("Unable to find cortical area to unwrap!".into())); // TODO specific error?
                }
                Some(mapped_data) => {
                    ordered_refs[index].write_xyzp_neuron_arrays(mapped_data, channel_index)?;
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

