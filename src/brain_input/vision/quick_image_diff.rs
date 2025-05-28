//! Quick image difference processing for FEAGI vision input.
//! 
//! This module provides the `QuickImageDiff` struct for efficient frame-to-frame
//! difference detection and processing. It combines image preprocessing, difference
//! calculation, segmented vision processing, and neuron data export into a single
//! streamlined pipeline for real-time vision processing in FEAGI.

use crate::cortical_data::CorticalID;
use super::image_frame::ImageFrame;
use super::segmented_vision_frame::SegmentedVisionFrame;
use super::descriptors::{FrameProcessingParameters, ChannelFormat, ColorSpace, SegmentedVisionCenterProperties, SegmentedVisionTargetResolutions};
use crate::error::DataProcessingError;
use crate::neuron_data::{CorticalMappedNeuronData, NeuronXYCPArrays};

/// A high-performance image difference processor for FEAGI vision input.
/// 
/// `QuickImageDiff` provides an efficient pipeline for processing consecutive image frames
/// to detect changes and convert them into neuron data for FEAGI. It maintains internal
/// buffers for frame comparison, applies configurable preprocessing, performs segmented
/// vision processing, and exports the results as serialized neuron data.
/// 
/// The processor alternates between two internal frame buffers to calculate differences,
/// applies optional preprocessing (cropping, resizing, brightness/contrast adjustment),
/// segments the difference image into nine regions for peripheral vision simulation,
/// and converts the results to neuron data organized by cortical areas.
/// 
/// # Examples
/// 
/// ```
/// use feagi_core_data_structures_and_processing::brain_input::vision::quick_image_diff::QuickImageDiff;
/// use feagi_core_data_structures_and_processing::brain_input::vision::descriptors::*;
/// 
/// let mut params = FrameProcessingParameters::new();
/// params.set_resizing_to((320, 240));
/// 
/// let resolutions = SegmentedVisionTargetResolutions::create_with_same_sized_peripheral(
///     (64, 64), (32, 32)
/// ).unwrap();
/// 
/// let center_props = SegmentedVisionCenterProperties::create_default_centered();
/// 
/// let mut processor = QuickImageDiff::new_from_preprocessor(
///     params,
///     &ChannelFormat::RGB,
///     &ColorSpace::Gamma,
///     resolutions,
///     center_props,
///     0
/// ).unwrap();
/// ```
pub struct QuickImageDiff{
    /// First frame buffer for difference calculation
    diff_frame_a: ImageFrame,
    /// Second frame buffer for difference calculation
    diff_frame_b: ImageFrame,
    /// Output frame containing the calculated difference
    diff_frame_out: ImageFrame,
    /// Segmented vision frame for peripheral vision processing
    outputted_segmented_frame: SegmentedVisionFrame,
    /// Flag indicating which frame buffer to use next (alternates between A and B)
    flag_a_subtract_from_b: bool,
    /// Optional preprocessing parameters to apply to incoming frames
    preprocessing_parameters: Option<FrameProcessingParameters>,
    /// Center properties for segmented vision processing
    segmented_center_coordinates: SegmentedVisionCenterProperties,
    /// Cached cortical mapped neuron data for efficient reuse
    cortical_mapped_data: Option<CorticalMappedNeuronData>,
    /// Ordered cortical IDs for the nine vision segments
    ordered_cortical_ids: [CorticalID; 9],
}

impl QuickImageDiff {
    
    /// Creates a new QuickImageDiff processor with preprocessing configuration.
    /// 
    /// This constructor sets up the complete processing pipeline with the specified
    /// preprocessing parameters, output format, segmentation configuration, and camera index.
    /// 
    /// # Arguments
    /// 
    /// * `preprocessor_parameters` - Image preprocessing configuration (cropping, resizing, etc.)
    /// * `output_color_channels` - The color channel format for output frames
    /// * `output_color_space` - The color space for output frames
    /// * `output_segment_resolutions` - Target resolutions for each of the nine segments
    /// * `segmentation_center_properties` - Configuration for the center region positioning
    /// * `camera_index` - Camera identifier for cortical area naming
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(QuickImageDiff) if the processor was created successfully
    /// - Err(DataProcessingError) if the configuration is invalid
    /// 
    /// # Errors
    /// 
    /// This method will return an error if:
    /// - Preprocessing specifies grayscale conversion but output is configured for color
    /// - No cropping or resizing parameters are specified (required for determining output size)
    /// - Any internal component creation fails
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::quick_image_diff::QuickImageDiff;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::descriptors::*;
    /// 
    /// let mut params = FrameProcessingParameters::new();
    /// params.set_resizing_to((320, 240));
    /// 
    /// let resolutions = SegmentedVisionTargetResolutions::create_with_same_sized_peripheral(
    ///     (64, 64), (32, 32)
    /// ).unwrap();
    /// 
    /// let processor = QuickImageDiff::new_from_preprocessor(
    ///     params,
    ///     &ChannelFormat::RGB,
    ///     &ColorSpace::Gamma,
    ///     resolutions,
    ///     SegmentedVisionCenterProperties::create_default_centered(),
    ///     0
    /// ).unwrap();
    /// ```
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
    
    /// Processes an incoming image frame and returns serialized neuron data.
    /// 
    /// This method performs the complete processing pipeline:
    /// 1. Applies preprocessing to the incoming frame
    /// 2. Calculates the difference with the previous frame
    /// 3. Updates the segmented vision frame with the difference
    /// 4. Converts the segments to neuron data
    /// 5. Serializes the neuron data to bytes
    /// 
    /// The processor alternates between two internal frame buffers to maintain
    /// the previous frame for difference calculation without requiring any memory reallocation.
    /// 
    /// # Arguments
    /// 
    /// * `incoming_image_frame` - The new image frame to process
    /// * `pixel_threshold` - Minimum pixel difference to register as a change (0-255)
    /// * `camera_index` - Camera identifier for cortical area naming
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(Vec<u8>) with the serialized neuron data
    /// - Err(DataProcessingError) if any processing step fails
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::quick_image_diff::QuickImageDiff;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::image_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::descriptors::*;
    /// 
    /// // Assuming processor is already created...
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(640, 480));
    /// // let neuron_bytes = processor.process_incoming_image_to_proper_diff_image(frame, 10, 0).unwrap();
    /// ```
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

        let cortical_mapped_data_ref = self.cortical_mapped_data.as_ref().unwrap();
        NeuronXYCPArrays::cortical_mapped_neuron_data_to_bytes(cortical_mapped_data_ref)
    }
}