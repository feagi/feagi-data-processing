//! Quick image difference detection processor for FEAGI vision processing.
//!
//! This module provides the `ImageFrameQuickDiffProcessor` which computes the difference
//! between consecutive image frames to detect motion or changes in the visual field.
//! It implements a threshold-based difference algorithm that outputs pixels above
//! the difference threshold, making it useful for motion detection and change analysis.

use std::fmt::Display;
use std::time::Instant;
use ndarray::{Array3, Zip};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant, ImageFrame};
use crate::io_data::image_descriptors::ImageFrameProperties;
use crate::io_processing::StreamCacheProcessor;

/// A stream processor that computes pixel-wise differences between consecutive image frames.
///
/// This processor maintains two internal image buffers and alternates between them to compute
/// the difference between the current input frame and the previous frame. The difference
/// calculation uses a threshold to filter out noise and small variations, outputting only
/// pixels where the absolute difference exceeds the specified threshold.
///
/// # Algorithm
///
/// For each pixel position (x, y, c):
/// - If `|current_pixel - previous_pixel| > threshold`: output = current_pixel
/// - Otherwise: output = 0.0
///
/// # Use Cases
///
/// - **Motion Detection**: Identify areas of movement in video streams
/// - **Change Detection**: Detect significant changes in static scenes  
/// - **Noise Filtering**: Filter out small variations while preserving significant changes
/// - **Event Triggering**: Generate events when visual changes exceed thresholds
///
/// # Example
///
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ColorChannelLayout};
/// use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameQuickDiffProcessor;
///
/// let props = ImageFrameProperties::new((640, 480), ColorSpace::Linear, ColorChannelLayout::RGB).unwrap();
/// let threshold = 0.1; // 10% difference threshold
/// let diff_processor = ImageFrameQuickDiffProcessor::new(props, threshold).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ImageFrameQuickDiffProcessor {
    /// The output buffer containing the computed difference image
    diff_cache: IOTypeData, // Image Frame
    /// First internal buffer for alternating frame storage
    cached_a: IOTypeData, // Image Frame
    /// Second internal buffer for alternating frame storage
    cached_b: IOTypeData, // Image Frame
    /// Properties that input images must match (resolution, color space, channels)
    input_definition: ImageFrameProperties,
    /// Flag indicating which buffer to use for the next comparison
    is_diffing_against_b: bool,
    /// Minimum difference threshold for pixel changes to be considered significant
    threshold: f32,
}

impl Display for ImageFrameQuickDiffProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageFrameQuickDiffProcessor()")
    }
}

impl StreamCacheProcessor for ImageFrameQuickDiffProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.input_definition))
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.input_definition))
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.diff_cache
    }

    fn process_new_input(&mut self, value: &IOTypeData, _time_of_input: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        if self.is_diffing_against_b {
            self.cached_a = value.clone();
            quick_diff(&self.cached_a, &self.cached_b, &mut self.diff_cache, self.threshold);
        }
        else {
            self.cached_b = value.clone();
            quick_diff(&self.cached_b, &self.cached_a, &mut self.diff_cache, self.threshold);
        }
        self.is_diffing_against_b = !self.is_diffing_against_b;
        Ok(&self.diff_cache)
    }
}

impl ImageFrameQuickDiffProcessor {
    /// Creates a new ImageFrameQuickDiffProcessor with specified properties and threshold.
    ///
    /// Initializes the processor with three internal image buffers (two for alternating storage
    /// and one for the output difference) all matching the specified properties. The threshold
    /// determines the minimum pixel difference required for changes to be considered significant.
    ///
    /// # Arguments
    ///
    /// * `image_properties` - Properties defining the input image format (resolution, color space, channels)
    /// * `threshold` - Minimum difference threshold for pixel changes (must be positive)
    ///
    /// # Returns
    ///
    /// * `Ok(ImageFrameQuickDiffProcessor)` - Successfully created processor
    /// * `Err(FeagiDataProcessingError)` - If threshold is negative or image creation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ColorChannelLayout};
    /// use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameQuickDiffProcessor;
    ///
    /// let props = ImageFrameProperties::new((320, 240), ColorSpace::Linear, ColorChannelLayout::RGB).unwrap();
    /// let processor = ImageFrameQuickDiffProcessor::new(props, 0.05).unwrap();
    /// ```
    pub fn new(image_properties: ImageFrameProperties, threshold: f32) -> Result<Self, FeagiDataProcessingError> {
        if threshold < 0.0 {
            return Err(IODataError::InvalidParameters("Threshold must be positive!".into()).into());
        }
        
        let cache_image = ImageFrame::from_image_frame_properties(&image_properties)?;
        Ok(ImageFrameQuickDiffProcessor {
            diff_cache: IOTypeData::ImageFrame(cache_image.clone()),
            cached_a: IOTypeData::ImageFrame(cache_image.clone()), // Image Frame
            cached_b: IOTypeData::ImageFrame(cache_image.clone()), // Image Frame
            input_definition: image_properties,
            is_diffing_against_b: false,
            threshold,
        })
    }
}

/// Computes the pixel-wise difference between two image frames with threshold filtering.
///
/// This function performs element-wise comparison between two source images and writes
/// the result to the output buffer. For each pixel, if the absolute difference exceeds
/// the threshold, the current pixel value is preserved; otherwise, the pixel is set to zero.
///
/// # Algorithm
///
/// For each pixel (x, y, channel):
/// ```text
/// difference = source_pixel - comparison_pixel
/// if difference > threshold:
///     output_pixel = source_pixel
/// else:
///     output_pixel = 0.0
/// ```
///
/// # Arguments
///
/// * `source` - The current input image frame
/// * `source_diffing` - The previous image frame to compare against  
/// * `diff_overwriting` - The output buffer to write the difference result
/// * `threshold` - Minimum difference required to preserve a pixel value
///
/// # Returns
///
/// * `Ok(())` - If the difference computation was successful
/// * `Err(FeagiDataProcessingError)` - If type conversion or data access fails
///
/// # Performance
///
/// This function uses ndarray's parallel iteration capabilities for efficient
/// pixel-wise operations across potentially large image arrays.
fn quick_diff(source: &IOTypeData, source_diffing: &IOTypeData, diff_overwriting: &mut IOTypeData, threshold: f32) -> Result<(), FeagiDataProcessingError> {
    let read_from: &ImageFrame = source.try_into()?;
    let source_diff_from: &ImageFrame = source_diffing.try_into()?;
    let write_to: &mut ImageFrame = diff_overwriting.try_into()?;

    let read_from: &Array3<f32> = read_from.get_internal_data();
    let source_diff_from: &Array3<f32> = source_diff_from.get_internal_data();
    let write_to: &mut Array3<f32> = write_to.get_internal_data_mut();
    
    Zip::from(write_to).and(read_from).and(source_diff_from).for_each(|w, &r, &s| {
        let x = r - s;
        if x > threshold {
            *w = r;
        }
        else {
            *w = 0f32;
        }
    });
    
    Ok(())
}

