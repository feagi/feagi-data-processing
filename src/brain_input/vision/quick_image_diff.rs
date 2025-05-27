use crate::cortical_data::CorticalID;
use super::image_frame::ImageFrame;
use super::segmented_vision_frame::SegmentedVisionFrame;
use super::descriptors::{FrameProcessingParameters, ChannelFormat, ColorSpace, SegmentedVisionCenterProperties, SegmentedVisionTargetResolutions};
use crate::error::DataProcessingError;
use crate::neuron_data::{CorticalMappedNeuronData, NeuronXYCPArrays};

pub struct QuickImageDiff{
    diff_frame_a: ImageFrame,
    diff_frame_b: ImageFrame,
    diff_frame_out: ImageFrame,
    outputted_segmented_frame: SegmentedVisionFrame,
    flag_a_subtract_from_b: bool,
    preprocessing_parameters: Option<FrameProcessingParameters>,
    segmented_center_coordinates: SegmentedVisionCenterProperties,
    cortical_mapped_data: Option<CorticalMappedNeuronData>,
    ordered_cortical_ids: [CorticalID; 9],
}

impl QuickImageDiff {
    
    pub fn new_from_preprocessor(preprocessor_parameters: FrameProcessingParameters,
                                 output_color_channels: &ChannelFormat, output_color_space: &ColorSpace,
                                 output_segment_resolutions: SegmentedVisionTargetResolutions,
                                 segmentation_center_properties: SegmentedVisionCenterProperties,
                                 camera_index: u8)
                                 -> Result<Self, DataProcessingError> {
        if preprocessor_parameters.convert_to_grayscale && output_color_channels != &ChannelFormat::GrayScale {
            return Err(DataProcessingError::InvalidInputBounds("Preprocessor specifies conversion of image into grayscale, while output is defined as being in color!".into()));
        }
        let final_total_width_height = preprocessor_parameters.get_final_width_height();
        if final_total_width_height.is_err() {
            return Err(DataProcessingError::MissingContext("You must define a cropping and/or a resize parameter for the preprocessor!".into()));
        }
        let final_total_width_height = final_total_width_height.unwrap();

        Ok(QuickImageDiff {
            diff_frame_a: ImageFrame::new(&output_color_channels, &output_color_space, &final_total_width_height),
            diff_frame_b: ImageFrame::new(&output_color_channels, &output_color_space, &final_total_width_height),
            diff_frame_out: ImageFrame::new(&output_color_channels, &output_color_space, &final_total_width_height),
            outputted_segmented_frame: SegmentedVisionFrame::new(
                &output_segment_resolutions, output_color_channels, output_color_space, final_total_width_height
            )?,
            flag_a_subtract_from_b: false,
            preprocessing_parameters: Some(preprocessor_parameters),
            segmented_center_coordinates: segmentation_center_properties,
            cortical_mapped_data: None,
            ordered_cortical_ids: SegmentedVisionFrame::create_ordered_cortical_ids(camera_index, output_color_channels == &ChannelFormat::GrayScale)?,
        })
    }
    
    
    pub fn process_incoming_image_to_proper_diff_image(&mut self, incoming_image_frame: ImageFrame, pixel_threshold: u8, camera_index: u8) -> Result<Vec<u8>, DataProcessingError> {
        
        // handle preprocessing
        if self.preprocessing_parameters.is_some() {
            if self.flag_a_subtract_from_b {
                self.diff_frame_a.in_place_run_processor(self.preprocessing_parameters.unwrap(), incoming_image_frame)?;
            }
            else {
                self.diff_frame_b.in_place_run_processor(self.preprocessing_parameters.unwrap(), incoming_image_frame)?;
            }
        }
        else { // no processing
            if self.flag_a_subtract_from_b {
                self.diff_frame_a = incoming_image_frame;
            }
            else {
                self.diff_frame_b = incoming_image_frame;
            }
        }
        
        // diff
        if self.flag_a_subtract_from_b {
            self.diff_frame_out.in_place_calculate_difference_thresholded(&self.diff_frame_a, &self.diff_frame_b, pixel_threshold)?;
        }
        else {
            self.diff_frame_out.in_place_calculate_difference_thresholded(&self.diff_frame_b, &self.diff_frame_a, pixel_threshold)?;
        }
        self.flag_a_subtract_from_b = !self.flag_a_subtract_from_b;
        
        
        // Update segments
        self.outputted_segmented_frame.update_segments(&self.diff_frame_out, self.segmented_center_coordinates)?;
        
        // generate cortical data
        if self.cortical_mapped_data.is_none() {
            self.cortical_mapped_data = Some(self.outputted_segmented_frame.export_as_new_cortical_mapped_neuron_data(camera_index)?);
        }
        else{
            let cortical_mapped_data_ref = self.cortical_mapped_data.as_mut().unwrap();
            self.outputted_segmented_frame.inplace_export_cortical_mapped_neuron_data(&self.ordered_cortical_ids, cortical_mapped_data_ref)?;
        }
        
        // TODO this can be cached
        let cortical_mapped_data = self.outputted_segmented_frame.export_as_new_cortical_mapped_neuron_data(camera_index)?;

        let cortical_mapped_data_ref = self.cortical_mapped_data.as_mut().unwrap(); // TODO we don't need this to be mutable!
        NeuronXYCPArrays::cortical_mapped_neuron_data_to_bytes(cortical_mapped_data_ref)
    }
}