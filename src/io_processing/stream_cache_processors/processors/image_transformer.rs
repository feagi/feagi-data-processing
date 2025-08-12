//! Image transformation stream processor for FEAGI vision processing pipelines.
//!
//! This module provides the `ImageFrameTransformerProcessor` which applies a configured
//! set of image transformations to incoming frames in a stream processing pipeline.
//! It wraps an `ImageFrameTransformerDefinition` to provide streaming functionality
//! with caching and efficient processing.

use std::fmt::Display;
use std::time::Instant;
use crate::error::FeagiDataProcessingError;
use crate::io_data::{IOTypeData, IOTypeVariant, ImageFrame, ImageFrameTransformerDefinition};
use crate::io_processing::StreamCacheProcessor;

/// A stream processor that applies image transformations to incoming frames.
///
/// This processor wraps an `ImageFrameTransformerDefinition` to provide streaming
/// functionality for image transformation pipelines. It maintains an internal cache
/// of the most recent output and applies the configured transformations (cropping,
/// resizing, color adjustments, etc.) to each incoming frame.
///
/// # Transformation Pipeline
///
/// The processor applies transformations in the optimal order defined by the
/// underlying `ImageFrameTransformerDefinition`:
/// 1. **Cropping** - Extract regions of interest
/// 2. **Resizing** - Scale to target dimensions  
/// 3. **Color space conversion** - Convert between Linear/Gamma
/// 4. **Brightness adjustment** - Modify pixel intensity
/// 5. **Contrast adjustment** - Adjust image contrast
/// 6. **Grayscale conversion** - Convert to single-channel
///
/// # Performance
///
/// The processor includes optimized fast paths for common transformation combinations
/// and maintains a pre-allocated output buffer to minimize memory allocations during
/// stream processing.
///
/// # Example
///
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::{ImageFrameTransformerDefinition};
/// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ColorChannelLayout};
/// use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameTransformerProcessor;
///
/// let input_props = ImageFrameProperties::new((640, 480), ColorSpace::Linear, ColorChannelLayout::RGB).unwrap();
/// let mut transformer_def = ImageFrameTransformerDefinition::new(input_props);
/// transformer_def.set_resizing_to((224, 224)).unwrap();
/// transformer_def.set_conversion_to_grayscale().unwrap();
///
/// let processor = ImageFrameTransformerProcessor::new(transformer_def).unwrap();
/// ```
#[derive(Debug)]
pub struct ImageFrameTransformerProcessor {
    /// The transformation configuration defining which operations to apply and their parameters
    transformer_definition: ImageFrameTransformerDefinition,
    /// Cached output buffer containing the most recent transformed image
    cached: IOTypeData, // Image Frame
}

impl Display for ImageFrameTransformerProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageFrameTransformerProcessor({:?})", self.transformer_definition)
    }
}

impl StreamCacheProcessor for ImageFrameTransformerProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(*self.transformer_definition.get_input_image_properties()))
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.transformer_definition.get_output_image_properties()))
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.cached
    }

    fn process_new_input(&mut self, value: &IOTypeData, _time_of_input: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let read_from: &ImageFrame = value.try_into()?;
        let write_target: &mut ImageFrame = (&mut self.cached).try_into()?;
        self.transformer_definition.process_image(read_from, write_target)?;
        Ok(&self.cached)
    }
}

impl ImageFrameTransformerProcessor {
    /// Creates a new ImageFrameTransformerProcessor with the specified transformation definition.
    ///
    /// Initializes the processor with a pre-allocated output buffer sized according to the
    /// expected output properties of the transformation pipeline. The buffer is created based
    /// on the final resolution, color space, and channel layout after all transformations.
    ///
    /// # Arguments
    ///
    /// * `transformer_definition` - Configuration defining the transformation pipeline to apply
    ///
    /// # Returns
    ///
    /// * `Ok(ImageFrameTransformerProcessor)` - Successfully created processor
    /// * `Err(FeagiDataProcessingError)` - If output buffer allocation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::{ImageFrameTransformerDefinition};
    /// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ColorChannelLayout};
    /// use feagi_core_data_structures_and_processing::io_processing::processors::ImageFrameTransformerProcessor;
    ///
    /// let input_props = ImageFrameProperties::new((1920, 1080), ColorSpace::Linear, ColorChannelLayout::RGB).unwrap();
    /// let mut transformer_def = ImageFrameTransformerDefinition::new(input_props);
    /// transformer_def.set_cropping_from((100, 100), (900, 700)).unwrap();
    /// transformer_def.set_resizing_to((256, 256)).unwrap();
    ///
    /// let processor = ImageFrameTransformerProcessor::new(transformer_def).unwrap();
    /// ```
    pub fn new(transformer_definition: ImageFrameTransformerDefinition) -> Result<Self, FeagiDataProcessingError> {
        Ok(ImageFrameTransformerProcessor{
            cached: IOTypeData::ImageFrame(ImageFrame::from_image_frame_properties(&transformer_definition.get_output_image_properties())?),
            transformer_definition,
        })
    }
}