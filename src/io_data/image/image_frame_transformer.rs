//! Image transformation pipeline definitions for FEAGI vision processing.
//!
//! This module provides the `ImageFrameTransformerDefinition` struct which defines
//! a complete image processing pipeline including cropping, resizing, color space
//! conversion, brightness/contrast adjustment, and grayscale conversion. The 
//! transformations are applied in a specific order for optimal performance.

use ndarray::{s, ArrayView3};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::image_descriptors::{ColorChannelLayout, ColorSpace, CornerPoints, ImageFrameProperties};
use crate::io_data::ImageFrame;

/// Defines a complete image transformation pipeline with multiple processing steps.
///
/// This structure configures a series of image processing operations that are applied
/// in a specific order to transform input images for FEAGI vision processing. The
/// operations are applied in the following sequence:
///
/// 1. **Cropping**: Extract a specific region from the input image
/// 2. **Resizing**: Scale the image to a target resolution
/// 3. **Color space conversion**: Convert between Linear and Gamma color spaces
/// 4. **Brightness adjustment**: Multiply pixel values by a brightness factor
/// 5. **Contrast adjustment**: Adjust image contrast
/// 6. **Grayscale conversion**: Convert RGB/RGBA images to grayscale
///
/// # Performance Considerations
///
/// The implementation includes optimized fast paths for common operation combinations
/// (such as crop+resize+grayscale) to minimize computational overhead. When specific
/// combinations are detected, specialized functions are used instead of applying
/// each operation sequentially.
///
/// # Example
///
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::{ImageFrameTransformer};
/// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ColorSpace, ColorChannelLayout, ImageFrameProperties};
///
/// let input_props = ImageFrameProperties::new((640, 480), ColorSpace::Linear, ColorChannelLayout::RGB).unwrap();
/// let mut transformer = ImageFrameTransformer::new(input_props);
///
/// // Configure the transformation pipeline
/// transformer.set_cropping_from((100, 100), (540, 380)).unwrap();
/// transformer.set_resizing_to((224, 224)).unwrap();
/// transformer.set_conversion_to_grayscale(true).unwrap();
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImageFrameTransformer {
    /// Properties that the input image must match (resolution, color space, channel layout)
    input_image_properties: ImageFrameProperties,
    /// Optional cropping region defined by corner points
    cropping_from: Option<CornerPoints>, 
    /// Optional target resolution for resizing operation
    final_resize_xy_to: Option<(usize, usize)>,
    /// Optional target color space for conversion
    convert_color_space_to: Option<ColorSpace>,
    /// Optional brightness multiplier factor
    multiply_brightness_by: Option<f32>,
    /// Optional contrast adjustment factor
    change_contrast_by: Option<f32>,
    /// Whether to convert the image to grayscale (only allowed on RGB/RGBA images)
    convert_to_grayscale: bool,
}

impl std::fmt::Display for ImageFrameTransformer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let initial = format!("Expecting {}.", self.input_image_properties);
        let mut steps: String = match (self.cropping_from, self.final_resize_xy_to) {
            (None, None) => format!("Keeping input size of <{}, {}> (no cropping from or resizing to)", self.input_image_properties.get_expected_xy_resolution().0, self.input_image_properties.get_expected_xy_resolution().1),
            (Some(cropping_from), None) => format!("Cropping from xy points <{}, {}> to <{}, {}> without resizing after,",
                                                   cropping_from.lower_left_row_major().1, cropping_from.lower_left_row_major().0, cropping_from.upper_right_row_major().1, cropping_from.upper_right_row_major().0),
            (None, Some(final_resize_xy_to)) => format!("resizing to resolution <{}, {}> without any cropping,", final_resize_xy_to.0, final_resize_xy_to.1),
            (Some(cropping_from), Some(final_resize_xy_to)) => format!("Cropping from xy points <{}, {}> to <{}, {}> then resizing to resolution <{}, {}>,",
                                                                       cropping_from.lower_left_row_major().1, cropping_from.lower_left_row_major().0, cropping_from.upper_right_row_major().1, cropping_from.upper_right_row_major().0, final_resize_xy_to.0, final_resize_xy_to.1),
        };
        steps += &*(match self.convert_color_space_to {
            None => String::new(),
            Some(change_colorspace_to) => format!("Convert Colorspace to {}", change_colorspace_to.to_string()),
        });
        steps += &*(match self.multiply_brightness_by {
            None => String::new(),
            Some(multiply_brightness_by) => format!("Multiply brightness by {}", multiply_brightness_by),
        });
        steps += &*(match self.change_contrast_by {
            None => String::new(),
            Some(change_contrast_by) => format!("Change contrast by {}", change_contrast_by),
        });
        steps += &*(match self.convert_to_grayscale {
            false => String::new(),
            true => "Convert to grayscale".to_string(),
        });
        write!(f, "ImageFrameCleanupDefinition({} {})", initial, steps)
    }
}

impl ImageFrameTransformer {
    
    /// Creates a new image transformer definition with specified input requirements.
    ///
    /// Initializes a transformer with the required input image properties and all
    /// transformation options disabled. Use the setter methods to configure specific
    /// transformations as needed.
    ///
    /// # Arguments
    ///
    /// * `input_image_properties` - The required properties for input images (resolution, color space, channels)
    ///
    /// # Returns
    ///
    /// A new `ImageFrameTransformerDefinition` with no transformations configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ColorChannelLayout};
    /// use feagi_core_data_structures_and_processing::io_data::ImageFrameTransformer;
    ///
    /// let props = ImageFrameProperties::new((640, 480), ColorSpace::Linear, ColorChannelLayout::RGB);
    /// let transformer = ImageFrameTransformer::new(props.unwrap());
    /// ```
    pub fn new(input_image_properties: ImageFrameProperties) -> ImageFrameTransformer {
        ImageFrameTransformer {
            input_image_properties,
            cropping_from: None,
            final_resize_xy_to: None,
            multiply_brightness_by: None,
            change_contrast_by: None,
            convert_color_space_to: None,
            convert_to_grayscale: false,
        }
    }

    pub fn new_from_input_output_properties(input: &ImageFrameProperties, output: &ImageFrameProperties) -> Result<Self, FeagiDataProcessingError> {
        let mut definition = ImageFrameTransformer::new(input.clone());
        if output.get_expected_color_channel_layout() != input.get_expected_color_channel_layout() {
            if output.get_expected_color_channel_layout() == ColorChannelLayout::GrayScale && input.get_expected_color_channel_layout() == ColorChannelLayout::RGB {
                // supported
                definition.convert_to_grayscale = true;
            }
            // unsupported
            return Err(IODataError::InvalidParameters("Given Color Conversion not possible!". into()). into())
        }
        if output.get_expected_xy_resolution() != input.get_expected_xy_resolution() {
            definition.set_resizing_to(output.get_expected_xy_resolution());
        }
        if output.get_expected_color_space() != output.get_expected_color_space() {
            definition.set_color_space_to(&output.get_expected_color_space());
        }
        Ok(definition)
    }

    /// Returns the required input image properties.
    ///
    /// # Returns
    ///
    /// A reference to the `ImageFrameProperties` that input images must match.
    pub fn get_input_image_properties(&self) -> &ImageFrameProperties { &self.input_image_properties }
    
    /// Calculates and returns the output image properties after all transformations.
    ///
    /// This method determines what the final image properties will be after applying
    /// all configured transformations, taking into account cropping, resizing, color
    /// space conversion, and grayscale conversion.
    ///
    /// # Returns
    ///
    /// An `ImageFrameProperties` struct representing the expected output properties.
    pub fn get_output_image_properties(&self) -> ImageFrameProperties {
        let resolution = match (self.cropping_from, self.final_resize_xy_to) {
            (None, None) => self.input_image_properties.get_expected_xy_resolution(),
            (Some(cropping_from), None) => cropping_from.enclosed_area_width_height(),
            (None, Some(final_resize_xy_to)) => final_resize_xy_to,
            (Some(_), Some(final_resize_xy_to)) => final_resize_xy_to,
        };
        let color_space = match self.convert_color_space_to {
            None => self.input_image_properties.get_expected_color_space(),
            Some(color_space_to) => color_space_to,
        };
        let color_channel_layout = match self.convert_to_grayscale {
            false => self.input_image_properties.get_expected_color_channel_layout(),
            true => ColorChannelLayout::GrayScale,
        };
        ImageFrameProperties::new(resolution, color_space, color_channel_layout).unwrap()
    }
    
    /// Verifies that an input image matches the required properties.
    ///
    /// Checks if the given image frame has the correct resolution, color space,
    /// and channel layout as specified in the input properties.
    ///
    /// # Arguments
    ///
    /// * `verifying_image` - The image frame to verify
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the image matches the required properties
    /// * `Err(FeagiDataProcessingError)` if the image doesn't match
    pub fn verify_input_image_allowed(&self, verifying_image: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        self.input_image_properties.verify_image_frame_matches_properties(verifying_image)
    }
    

    // TODO 2 / 4 channel pipelines!
    // Due to image segmentor, I would argue the most common route is crop + resize + grayscale

    /// Processes an input image through the configured transformation pipeline.
    ///
    /// Applies all configured transformations to the source image and writes the result
    /// to the destination image. The transformations are applied in the optimal order
    /// for performance, with fast paths for common operation combinations.
    ///
    /// # Arguments
    ///
    /// * `source` - The input image to transform
    /// * `destination` - The output image to write the transformed result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the transformation was successful
    /// * `Err(FeagiDataProcessingError)` if any transformation step fails
    ///
    /// # Performance Notes
    ///
    /// This method includes optimized fast paths for common combinations like:
    /// - Crop + resize + grayscale (common in segmentation vision)
    /// - Copy only (no transformations)
    /// - Single operations (crop only, resize only, etc.)
    pub fn process_image(&self, source: &ImageFrame, destination: &mut ImageFrame) -> Result<(), FeagiDataProcessingError> {
        match self {
            // Do literally nothing, just copy the data
            ImageFrameTransformer {
                input_image_properties,
                cropping_from: None,
                final_resize_xy_to: None,
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by: None,
                convert_to_grayscale: false
            } => {
                *destination = source.clone();
                Ok(())
            }

            // Only cropping
            ImageFrameTransformer {
                input_image_properties,
                cropping_from: Some(cropping_from),
                final_resize_xy_to: None,
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                crop(source, destination, cropping_from, self.get_output_channel_count())
            }

            // Only resizing
            ImageFrameTransformer {
                input_image_properties,
                cropping_from: None,
                final_resize_xy_to: Some(final_resize_xy_to),
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                resize(source, destination, final_resize_xy_to)
            }

            // Only grayscaling
            ImageFrameTransformer {
                input_image_properties,
                cropping_from: None,
                final_resize_xy_to: None,
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: true
            } => {
                to_grayscale(source, destination, self.input_image_properties.get_expected_color_space())
            }

            // Cropping, Resizing
            ImageFrameTransformer {
                input_image_properties,
                cropping_from: Some(cropping_from),
                final_resize_xy_to: Some(final_resize_xy_to),
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                crop_and_resize(source, destination, cropping_from, final_resize_xy_to)
            }

            // Cropping, Resizing, Grayscaling (the most common with segmentation vision)
            ImageFrameTransformer {
                input_image_properties,
                cropping_from: Some(cropping_from),
                final_resize_xy_to: Some(final_resize_xy_to),
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: true
            } => {
                crop_and_resize_and_grayscale(source, destination, cropping_from, final_resize_xy_to, self.input_image_properties.get_expected_color_space())
            }

            // If no fast path, use this slower universal one
            _ => {
                // This function is much slower, There may be some optimization work possible, but ensure the most common step combinations have an accelerated path
                let is_cropping_is_resizing = (self.cropping_from, self.final_resize_xy_to);

                let mut processing = source.clone();
                match is_cropping_is_resizing {
                    (None, None) => {
                        // don't do anything
                    }
                    (Some(cropping_from), None) => {
                        crop(source, &mut processing, &cropping_from, self.get_output_channel_count())?;
                    }
                    (None, Some(final_resize_xy_to)) => {
                        resize(source, &mut processing, &final_resize_xy_to)?;
                    }
                    (Some(cropping_from), Some(final_resize_xy_to)) => {
                        crop_and_resize(source, &mut processing, &cropping_from, &final_resize_xy_to)?;
                    }
                };

                match self.convert_color_space_to {
                    None => {
                        // Do Nothing
                    }
                    Some(color_space) => {
                        return Err(FeagiDataProcessingError::NotImplemented)
                    }
                }

                match self.multiply_brightness_by {
                    None => {
                        // Do Nothing
                    }
                    Some(brightness_multiplier) => {
                        processing.change_brightness(brightness_multiplier)?;
                    }
                }

                match self.change_contrast_by {
                    None => {
                        // Do Nothing
                    }
                    Some(contrast_multiplier) => {
                        processing.change_contrast(contrast_multiplier)?;
                    }
                }

                if self.convert_to_grayscale {
                    return Err(FeagiDataProcessingError::NotImplemented)
                }

                *destination = processing;
                Ok(())
            }

        }


    }

    //region set settings
    // TODO safety bound checks!

    /// Sets the cropping region for the transformation pipeline.
    ///
    /// Configures the transformer to crop the input image to a specified rectangular region
    /// before applying other transformations. The cropping is performed using inclusive
    /// lower-left and exclusive upper-right coordinates in Cartesian space.
    ///
    /// # Arguments
    ///
    /// * `lower_left_xy_point_inclusive` - The lower-left corner (x, y) of the crop region (inclusive)
    /// * `upper_right_xy_point_exclusive` - The upper-right corner (x, y) of the crop region (exclusive)
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - Reference to self for method chaining
    /// * `Err(FeagiDataProcessingError)` - If the crop region is invalid
    pub fn set_cropping_from(&mut self, lower_left_xy_point_inclusive: (usize, usize), upper_right_xy_point_exclusive: (usize, usize)) -> Result<&mut Self, FeagiDataProcessingError> {
        let corner_points = CornerPoints::new_from_cartesian(lower_left_xy_point_inclusive, upper_right_xy_point_exclusive, self.input_image_properties.get_expected_xy_resolution())?;
        self.cropping_from = Some(corner_points);
        Ok(self)
    }

    /// Sets the target resolution for image resizing.
    ///
    /// Configures the transformer to resize the image (after any cropping) to the
    /// specified width and height using nearest neighbor interpolation.
    ///
    /// # Arguments
    ///
    /// * `new_xy_resolution` - Target resolution as (width, height) in pixels
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - Reference to self for method chaining
    /// * `Err(FeagiDataProcessingError)` - If the resolution is invalid
    pub fn set_resizing_to(&mut self, new_xy_resolution: (usize, usize)) -> Result<&mut Self, FeagiDataProcessingError> {
        self.final_resize_xy_to = Some(new_xy_resolution);
        Ok(self)
    }

    /// Sets the brightness multiplier for the transformation pipeline.
    ///
    /// Configures the transformer to multiply all pixel values by the specified
    /// brightness factor. Values greater than 1.0 increase brightness, while
    /// values between 0.0 and 1.0 decrease brightness.
    ///
    /// # Arguments
    ///
    /// * `brightness_multiplier` - Factor to multiply pixel values by (must be positive)
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - Reference to self for method chaining
    /// * `Err(FeagiDataProcessingError)` - If the multiplier is invalid
    pub fn set_brightness_multiplier(&mut self, brightness_multiplier: f32) -> Result<&mut Self, FeagiDataProcessingError> {
        if brightness_multiplier == 1.0 {
            self.multiply_brightness_by = None;
        }
        else {
            self.multiply_brightness_by = Some(brightness_multiplier);
        }
        Ok(self)
    }

    /// Sets the contrast adjustment factor for the transformation pipeline.
    ///
    /// Configures the transformer to adjust image contrast. Positive values increase
    /// contrast, negative values decrease contrast, and zero results in no change.
    ///
    /// # Arguments
    ///
    /// * `contrast_change` - Contrast adjustment factor (typically between -1.0 and 1.0)
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - Reference to self for method chaining
    /// * `Err(FeagiDataProcessingError)` - If the contrast factor is invalid
    pub fn set_contrast_change(&mut self, contrast_change: f32) -> Result<&mut Self, FeagiDataProcessingError> {
        if contrast_change == 1.0 {
            self.change_contrast_by = None;
        }
        else {
            self.change_contrast_by = Some(contrast_change);
        }
        Ok(self)
    }

    /// Sets the target color space for conversion.
    ///
    /// Configures the transformer to convert the image to the specified color space.
    /// This operation is performed after cropping and resizing but before brightness
    /// and contrast adjustments.
    ///
    /// # Arguments
    ///
    /// * `color_space` - Target color space (Linear or Gamma)
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - Reference to self for method chaining
    /// * `Err(FeagiDataProcessingError)` - If the conversion is not supported
    pub fn set_color_space_to(&mut self, color_space: &ColorSpace) -> Result<&mut Self, FeagiDataProcessingError> {
        if color_space == &self.input_image_properties.get_expected_color_space() {
            self.convert_color_space_to = None;
        }
        else {
            self.convert_color_space_to = Some(*color_space);
        }
        Ok(self)
    }


    pub fn set_conversion_to_grayscale(&mut self, convert_to_grayscale: bool) -> Result<&mut Self, FeagiDataProcessingError> {
        if self.input_image_properties.get_expected_color_channel_layout() == ColorChannelLayout::RG {
            return Err(FeagiDataProcessingError::NotImplemented)
        }
        self.convert_to_grayscale = convert_to_grayscale;
        Ok(self)
    }

    //region clear settings

    /// Clears all transformation settings, resetting to default state.
    ///
    /// Removes all configured transformations (cropping, resizing, brightness, contrast,
    /// color space conversion, and grayscale conversion), returning the transformer to
    /// its initial state where only the input properties are preserved.
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::ImageFrameTransformer;
    /// use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ImageFrameProperties, ColorSpace, ColorChannelLayout};
    ///
    /// let props = ImageFrameProperties::new((640, 480), ColorSpace::Linear, ColorChannelLayout::RGB).unwrap();
    /// let mut transformer = ImageFrameTransformer::new(props);
    ///
    /// // Configure some transformations
    /// transformer.set_resizing_to((224, 224)).unwrap();
    /// transformer.set_conversion_to_grayscale(true).unwrap();
    ///
    /// // Clear all transformations
    /// transformer.clear_all_transformations();
    /// ```
    pub fn clear_all_transformations(&mut self) -> &Self {
        self.cropping_from = None;
        self.final_resize_xy_to = None;
        self.convert_color_space_to = None;
        self.multiply_brightness_by = None;
        self.change_contrast_by = None;
        self.convert_to_grayscale = false;
        self
    }

    /// Clears the cropping transformation.
    ///
    /// Removes the configured cropping region, causing the transformer to process
    /// the entire input image without cropping.
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    pub fn clear_cropping(&mut self) -> &Self {
        self.cropping_from = None;
        self
    }

    /// Clears the resizing transformation.
    ///
    /// Removes the configured target resolution, causing the transformer to preserve
    /// the original image dimensions (after any cropping).
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    pub fn clear_resizing(&mut self) -> &Self {
        self.final_resize_xy_to = None;
        self
    }

    /// Clears the brightness adjustment.
    ///
    /// Removes the configured brightness multiplier, causing the transformer to
    /// preserve the original image brightness.
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    pub fn clear_brightness_adjustment(&mut self) -> &Self {
        self.multiply_brightness_by = None;
        self
    }

    /// Clears the contrast adjustment.
    ///
    /// Removes the configured contrast modification, causing the transformer to
    /// preserve the original image contrast.
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    pub fn clear_contrast_adjustment(&mut self) -> &Self {
        self.change_contrast_by = None;
        self
    }

    /// Clears the color space conversion.
    ///
    /// Removes the configured target color space, causing the transformer to
    /// preserve the original color space.
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    pub fn clear_color_space_conversion(&mut self) -> &Self {
        self.convert_color_space_to = None;
        self
    }

    /// Clears the grayscale conversion.
    ///
    /// Disables grayscale conversion, causing the transformer to preserve the
    /// original color channels.
    ///
    /// # Returns
    ///
    /// Reference to self for method chaining.
    pub fn clear_grayscale_conversion(&mut self) -> &Self {
        self.convert_to_grayscale = false;
        self
    }

    //endregion

    //endregion
    
    //region helpers

    fn get_output_channel_count(&self) -> usize {
        if self.convert_to_grayscale {
            return 1;
        }
        self.input_image_properties.get_expected_color_channel_layout().into()
    }
    

    //endregion
    

    
}

//region source destination processors

fn crop(source: &ImageFrame, destination: &mut ImageFrame, crop_from: &CornerPoints, number_output_color_channels: usize) -> Result<(), FeagiDataProcessingError> {
    let mut destination_data = destination.get_internal_data_mut();
    let sliced_array_view: ArrayView3<f32> = source.get_internal_data().slice(
        s![crop_from.lower_left_row_major().0 .. crop_from.upper_right_row_major().0,
                crop_from.lower_left_row_major().1 .. crop_from.upper_right_row_major().1 , 0..number_output_color_channels]
    );
    destination_data = &mut sliced_array_view.into_owned();
    Ok(())
}

fn resize(source: &ImageFrame, destination: &mut ImageFrame, resize_xy_to: &(usize, usize)) -> Result<(), FeagiDataProcessingError> {
    // Uses Nearest Neighbor. Not pretty but fast
    let source_resolution = source.get_internal_resolution();
    let dest_resolution_f: (f32, f32) = (resize_xy_to.0 as f32, resize_xy_to.1 as f32);
    let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32);

    let source_data = source.get_internal_data();
    let mut destination_data = destination.get_internal_data_mut();

    for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
        let nearest_neighbor_coordinate_y: usize = (((y as f32) / dest_resolution_f.1) * source_resolution_f.1).floor() as usize;
        let nearest_neighbor_coordinate_x: usize = (((x as f32) / dest_resolution_f.0) * source_resolution_f.0).floor() as usize;
        let nearest_neighbor_channel_value: f32 = source_data[(
            nearest_neighbor_coordinate_x,
            nearest_neighbor_coordinate_y,
            c)];
        *color_val = nearest_neighbor_channel_value;
    };
    Ok(())
}

fn to_grayscale(source: &ImageFrame, destination: &mut ImageFrame, output_color_space: ColorSpace) -> Result<(), FeagiDataProcessingError> {
    // NOTE: destination should be grayscale and source should be RGB or RGBA
    let source_data = source.get_internal_data();
    let mut destination_data = destination.get_internal_data_mut();
    let (r_scale, g_scale, b_scale) = match output_color_space {
        ColorSpace::Linear => {(0.2126f32, 0.7152f32, 0.072f32)} // Using formula from https://stackoverflow.com/questions/17615963/standard-rgb-to-grayscale-conversion
        ColorSpace::Gamma => {(0.299f32, 0.587f32, 0.114f32)}  // https://www.youtube.com/watch?v=uKeKuaJ4nlw (I forget)
    };
    // TODO look into premultiplied alpha handling!

    for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
        // TODO this is bad, we shouldnt be iterating over color channel and matching like this. Major target for optimization!
        if c != 0 { continue; }
        *color_val = r_scale * source_data[(y, x, 0)] + b_scale * source_data[(y, x, 1)] + g_scale * source_data[(y, x, 2)];
    }
    Ok(())
    

    
}

fn crop_and_resize(source: &ImageFrame, destination: &mut ImageFrame, crop_from: &CornerPoints, resize_xy_to: &(usize, usize)) -> Result<(), FeagiDataProcessingError> {

    let crop_resolution: (usize, usize) = crop_from.enclosed_area_width_height();
    let resolution_f: (f32, f32) = (resize_xy_to.0 as f32, resize_xy_to.1 as f32);
    let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

    let dist_factor_yx: (f32, f32) = (
        crop_resolution_f.1 / resolution_f.1,
        crop_resolution_f.0 / resolution_f.0);

    let upper_left_corner_offset_yx: (usize, usize) = (
        crop_from.upper_left_row_major().0,
        crop_from.upper_left_row_major().1,
    );

    let source_data = source.get_internal_data();
    let mut destination_data = destination.get_internal_data_mut();

    for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
        let nearest_neighbor_coordinate_from_source_y: usize = ((y as f32) * dist_factor_yx.0).floor() as usize;
        let nearest_neighbor_coordinate_from_source_x: usize = ((x as f32) * dist_factor_yx.1).floor() as usize;
        let nnc_y_with_offset = nearest_neighbor_coordinate_from_source_y + upper_left_corner_offset_yx.0;
        let nnc_x_with_offset = nearest_neighbor_coordinate_from_source_x + upper_left_corner_offset_yx.1;

        let nearest_neighbor_channel_value: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            c)];
        *color_val = nearest_neighbor_channel_value;
    };
    Ok(())
}

fn crop_and_resize_and_grayscale(source: &ImageFrame, destination: &mut ImageFrame, crop_from: &CornerPoints, resize_xy_to: &(usize, usize), output_color_space: ColorSpace) -> Result<(), FeagiDataProcessingError> {

    let crop_resolution: (usize, usize) = crop_from.enclosed_area_width_height();
    let resolution_f: (f32, f32) = (resize_xy_to.0 as f32, resize_xy_to.1 as f32);
    let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

    let dist_factor_yx: (f32, f32) = (
        crop_resolution_f.1 / resolution_f.1,
        crop_resolution_f.0 / resolution_f.0);

    let upper_left_corner_offset_yx: (usize, usize) = (
        crop_from.upper_left_row_major().0,
        crop_from.upper_left_row_major().1,
    );

    let source_data = source.get_internal_data();
    let mut destination_data = destination.get_internal_data_mut();
    let (r_scale, g_scale, b_scale) = match output_color_space {
        ColorSpace::Linear => {(0.2126f32, 0.7152f32, 0.072f32)} // Using formula from https://stackoverflow.com/questions/17615963/standard-rgb-to-grayscale-conversion
        ColorSpace::Gamma => {(0.299f32, 0.587f32, 0.114f32)}
    };
    // TODO look into premultiplied alpha handling!

    for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
        // TODO this is bad, we shouldnt be iterating over color channel and matching like this. Major target for optimization!
        if c != 0 { continue; }
        let nearest_neighbor_coordinate_from_source_y: usize = ((y as f32) * dist_factor_yx.0).floor() as usize;
        let nearest_neighbor_coordinate_from_source_x: usize = ((x as f32) * dist_factor_yx.1).floor() as usize;
        let nnc_y_with_offset = nearest_neighbor_coordinate_from_source_y + upper_left_corner_offset_yx.0;
        let nnc_x_with_offset = nearest_neighbor_coordinate_from_source_x + upper_left_corner_offset_yx.1;
        let nearest_neighbor_channel_r: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            0)];
        let nearest_neighbor_channel_g: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            1)];
        let nearest_neighbor_channel_b: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            2)];
        
        *color_val = r_scale * nearest_neighbor_channel_r + b_scale * nearest_neighbor_channel_g + g_scale * nearest_neighbor_channel_b;
    }
    Ok(())
    
}
//endregion
