//! Image frame processing and manipulation for FEAGI vision input.
//! 
//! This module provides the `ImageFrame` struct and associated functionality for handling
//! image data in various formats, color spaces, and memory layouts. It supports common
//! image processing operations like cropping, resizing, brightness/contrast adjustment,
//! and conversion to neuron data for FEAGI processing.

use ndarray::{s, Array3, ArrayView3};
use crate::io_data::image::descriptors::{ChannelLayout, ColorSpace, CornerPoints, FrameProcessingParameters, MemoryOrderLayout};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::CorticalIOChannelIndex;
use crate::neuron_data::xyzp::NeuronXYZPArrays;

/// Represents an image frame with pixel data and metadata for FEAGI vision processing.
/// 
/// An `ImageFrame` stores image data as a 3D array of f32 values along with information
/// about the color channel format and color space. The internal storage uses row-major
/// ordering (height, width, channels) for efficient processing.
#[derive(Clone, Debug)]
pub struct ImageFrame {
    /// The pixel data stored as a 3D array with dimensions (height, width, channels)
    pixels: Array3<f32>,
    /// The color channel format (GrayScale, RG, RGB, or RGBA)
    channel_layout: ChannelLayout,
    /// The color space (Linear or Gamma)
    color_space: ColorSpace,
}

impl std::fmt::Display for ImageFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageFrame(Width={}, Height={}, ColorChannelFormat={}, ColorSpace={}",
               self.get_cartesian_width_height().0,
               self.get_cartesian_width_height().1,
               self.channel_layout.to_string(),
               self.color_space.to_string())
    }
}

impl ImageFrame {
    /// The internal memory layout used for storing pixel data
    const INTERNAL_MEMORY_LAYOUT: MemoryOrderLayout = MemoryOrderLayout::HeightsWidthsChannels;

    // region: common constructors

    /// Creates a new ImageFrame with the specified channel format, color space, and resolution.
    ///
    /// # Arguments
    ///
    /// * `channel_format` - The color channel format for the image (GrayScale, RG, RGB, or RGBA)
    /// * `color_space` - The color space of the image (Linear or Gamma)
    /// * `xy_resolution` - The resolution of the image as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A new ImageFrame instance with all pixels initialized to zero.
    pub fn new(channel_format: &ChannelLayout, color_space: &ColorSpace, xy_resolution: &(usize, usize)) -> ImageFrame {
        ImageFrame {
            channel_layout: *channel_format,
            color_space: *color_space,
            pixels: Array3::<f32>::zeros((xy_resolution.1, xy_resolution.0, *channel_format as usize)),
        }
    }

    /// Creates an ImageFrame from an existing ndarray with the specified color space.
    ///
    /// # Arguments
    ///
    /// * `input` - A 3D array of f32 values representing the image pixels
    /// * `color_space` - The color space of the image (Linear or Gamma)
    /// * `source_memory_order` - The memory layout of the input array
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if the input array has a valid number of color channels (1-4)
    /// - Err(DataProcessingError) if the number of color channels is invalid
    pub fn from_array(input: Array3<f32>, color_space: ColorSpace, source_memory_order: MemoryOrderLayout) -> Result<ImageFrame, FeagiDataProcessingError> {
        let number_color_channels: usize = input.shape()[2];
        Ok(ImageFrame {
            pixels: ImageFrame::change_memory_order_to_row_major(input, source_memory_order),
            color_space,
            channel_layout: ChannelLayout::try_from(number_color_channels)?
        })
    }

    /// Creates an ImageFrame from an existing ndarray with optional processing steps.
    ///
    /// This function allows creating an ImageFrame with a series of optional processing steps
    /// such as cropping, resizing, brightness adjustment, and contrast adjustment.
    ///
    /// # Arguments
    ///
    /// * `source_color_space` - The color space of the input image (Linear or Gamma)
    /// * `image_processing` - Parameters defining the processing steps to apply
    /// * `input` - A 3D array of f32 values representing the image pixels
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if all processing steps were successful
    /// - Err(DataProcessingError) if any processing step fails
    pub fn from_array_with_processing(source_color_space: ColorSpace, image_processing: FrameProcessingParameters, input: Array3<f32>) -> Result<ImageFrame, FeagiDataProcessingError> {
        // Let us set the memory order correct first, this has 0 cost
        let processed_input = ImageFrame::change_memory_order_to_row_major(input, image_processing.memory_ordering_of_source);

        let processing_steps_required = image_processing.process_steps_required_to_run();

        // there are 2^6 permutations of these bools. I ain't writing them all out here. Let us stick to the most common ones
        // bool order is {cropping_from, resizing_to, multiply_brightness, contrast, to_grayscale, color_space}
        match processing_steps_required { // I can't believe this isn't Yandresim!
            (false, false, false, false, false, false) => {
                // No processing steps specified
                ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT)
            }

            (true, true, false, false, false, false) => {
                // crop from and resize to
                let source_frame = ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT);
                ImageFrame::create_from_source_frame_crop_and_resize(&source_frame?, &image_processing.cropping_from.unwrap(), &image_processing.resizing_to.unwrap())
            }

            _ => {
                // We do not have an optimized pathway, just do this sequentially (although this is considerably slower)
                let mut frame = ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT)?;

                if image_processing.cropping_from.is_some() {
                    let corner_points_cropping = &image_processing.cropping_from.unwrap();
                    let _ = frame.crop_to(corner_points_cropping)?;
                };

                if image_processing.resizing_to.is_some() {
                    let corner_points_resizing = &image_processing.resizing_to.unwrap();
                    let _ = frame.resize_nearest_neighbor(corner_points_resizing)?;
                };

                if image_processing.multiply_brightness_by.is_some() {
                    let brightness = image_processing.multiply_brightness_by.unwrap();
                    let _ = frame.change_brightness_multiplicative(brightness)?;
                };

                if image_processing.change_contrast_by.is_some() {
                    let change_contrast_by = image_processing.change_contrast_by.unwrap();
                    let _ = frame.change_contrast(change_contrast_by)?;
                };

                // TODO grayscale conversions and color space conversions!

                Ok(frame)
            }
        }
    }

    // endregion

    // region: get properties

    /// Returns true if two ImageFrames have the same channel count, resolution, and color space.
    ///
    /// # Arguments
    ///
    /// * `a` - First ImageFrame to compare
    /// * `b` - Second ImageFrame to compare
    ///
    /// # Returns
    ///
    /// True if both frames have identical channel count, resolution, and color space.
    pub fn do_resolutions_channel_depth_and_color_spaces_match(a: &ImageFrame, b: &ImageFrame) -> bool {
        a.get_color_channel_count() == b.get_color_channel_count() && a.color_space == b.color_space
    }

    /// Returns true if the given array has valid dimensions for an ImageFrame.
    ///
    /// An array is considered valid if:
    /// - It has between 1 and 4 color channels
    /// - It has non-zero width and height
    ///
    /// # Arguments
    ///
    /// * `array` - The array to validate
    ///
    /// # Returns
    ///
    /// True if the array dimensions are valid for an ImageFrame, false otherwise.

    pub fn is_array_valid_for_image_frame(array: &Array3<f32>) -> bool {
        let shape: &[usize] = array.shape();
        if shape[2] > 4 || shape[2] == 0 {
            return false;
        }
        if shape[0] == 0 || shape[1] == 0 {
            return false;
        }
        true
    }

    /// Returns a reference to the channel layout of this image.
    ///
    /// # Returns
    ///
    /// A reference to the ChannelLayout enum value representing the image's color channel format.
    pub fn get_channel_layout(&self) -> &ChannelLayout {
        &self.channel_layout
    }

    /// Returns a reference to the color space of this image.
    ///
    /// # Returns
    ///
    /// A reference to the ColorSpace enum value representing the image's color space.
    pub fn get_color_space(&self) -> &ColorSpace {
        &self.color_space
    }

    /// Returns the number of color channels in this ImageFrame.
    ///
    /// # Returns
    ///
    /// The number of color channels as an usize:
    /// - 1 for GrayScale
    /// - 2 for RG
    /// - 3 for RGB
    /// - 4 for RGBA
    pub fn get_color_channel_count(&self) -> usize {
        self.channel_layout as usize
    }

    /// Returns a read-only view of the pixel data.
    ///
    /// This provides access to the underlying 3D ndarray of pixel values.
    ///
    /// # Returns
    ///
    /// An ArrayView3<f32> containing the pixel data.
    pub fn get_pixels_view(&self) -> ArrayView3<f32> {
        self.pixels.view()
    }

    /// Returns the resolution of the image in cartesian space (width, height)
    ///
    /// # Returns
    ///
    /// A tuple of (width, height) representing the image dimensions in pixels.
    pub fn get_cartesian_width_height(&self) -> (usize, usize) {
        let shape: &[usize] = self.pixels.shape();
        (shape[1], shape[0]) // because nd array is row major, where coords are yx
    }

    /// Returns the internal resolution of the image in row-major order (height, width).
    ///
    /// This returns the resolution in the internal storage format, which is row-major
    /// (height, width) rather than cartesian (width, height).
    ///
    /// # Returns
    ///
    /// A tuple of (height, width) representing the image dimensions in pixels.
    pub fn get_internal_resolution(&self) -> (usize, usize) {
        let shape: &[usize] = self.pixels.shape();
        (shape[0], shape[1])
    }
    
    /// Returns the internal shape of the image array in row-major order.
    ///
    /// The shape is returned as (height, width, channels) representing the dimensions
    /// of the internal ndarray storage.
    ///
    /// # Returns
    ///
    /// A tuple of (height, width, channels) representing the array dimensions.
    pub fn get_internal_shape(&self) -> (usize, usize, usize) {
        let shape: &[usize] = self.pixels.shape();
        (shape[0], shape[1], shape[2])
    }

    /// Returns the maximum number of neurons that could be generated from this image.
    /// 
    /// This calculates the total number of pixels across all channels, which represents
    /// the maximum number of neurons that could be created if every pixel value was
    /// above the threshold when converting to neuron data.
    /// 
    /// # Returns
    /// 
    /// The maximum possible neuron count (width × height × channels).
    pub fn get_max_capacity_neuron_count(&self) -> usize {
        self.pixels.shape()[0] * self.pixels.shape()[1] * self.pixels.shape()[2]
    }

    // endregion

    // region: Modify frame

    /// Adjusts the brightness of the image by multiplying each pixel value by a positive factor.
    ///
    /// This method modifies the image in-place by scaling all pixel values by the given factor.
    /// Values are clamped to the range [0.0, 1.0] after multiplication.
    ///
    /// # Arguments
    ///
    /// * `brightness_factor` - The factor to multiply each pixel value by (must be positive)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(DataProcessingError) if the brightness factor is negative
    pub fn change_brightness_multiplicative(&mut self, brightness_factor: f32) -> Result<(), FeagiDataProcessingError> {
        if brightness_factor < 0.0 {
            return Err(IODataError::InvalidParameters("Multiply brightness by must be positive!".into()).into());
        }

        self.pixels.mapv_inplace(|v| {
            let scaled = (v) * brightness_factor;
            scaled.clamp(0.0, 1.0) // Ensure that we do not exceed outside 0.0 and 1.0
        });
        Ok(())
    }


    /// Adjusts the contrast of the image using a contrast factor.
    ///
    /// The contrast adjustment is performed using a standard contrast adjustment algorithm
    /// that preserves the middle gray value (0.5) while stretching or compressing the
    /// dynamic range of the image.
    ///
    /// # Arguments
    ///
    /// * `contrast_factor` - A value between -1.0 and 1.0 where:
    ///   - 1.0: Maximum contrast increase (dark values become darker, bright values become brighter)
    ///   - 0.0: No change
    ///   - -1.0: Maximum contrast decrease (all values become middle gray)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(DataProcessingError) if the contrast factor is outside the valid range of -1 to 1
    pub fn change_contrast(&mut self, contrast_factor: f32) -> Result<(), FeagiDataProcessingError> {
        if contrast_factor < -1.0 || contrast_factor > 1.0 {
            return Err(IODataError::InvalidParameters("The contrast factor must be between -1.0 and 1.0!".into()).into());
        }
        // Algo sourced from https://ie.nitk.ac.in/blog/2020/01/19/algorithms-for-adjusting-brightness-and-contrast-of-an-image/
        const CORRECTION_FACTOR: f32 = 1.015686; //  259 / 255
        self.pixels.mapv_inplace(|v| {
            let factor: f32 = (CORRECTION_FACTOR * (contrast_factor + 1.0)) / (CORRECTION_FACTOR - contrast_factor);
            let pixel_val: f32 = (factor * (v - 0.5)) + 0.5;
            pixel_val.clamp(0.0, 1.0)
        });
        Ok(())
    }

    // TODO Color Space Transformations

    // TODO to grayscale

    /// Crops the image to the specified region.
    ///
    /// This method modifies the image in-place by cropping it to the specified region
    /// defined by CornerPoints. This operation is not done inplace and
    /// instead allocates a new array.
    ///
    /// # Arguments
    ///
    /// * `corners_crop` - The CornerPoints defining the region to crop
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(&mut Self) if the crop operation was successful
    /// - Err(DataProcessingError) if the crop region would not fit in the image
    pub fn crop_to(&mut self, corners_crop: &CornerPoints) -> Result<&mut Self, FeagiDataProcessingError> {
        if !corners_crop.does_fit_in_frame_of_width_height(self.get_cartesian_width_height()) {
            return Err(IODataError::InvalidParameters("The given crop would not fit in the given source!".into()).into());
        }
        let sliced_array_view: ArrayView3<f32> =
            self.pixels.slice(s![corners_crop.upper_right_row_major().0 .. corners_crop.lower_left_row_major().0,
                corners_crop.lower_left_row_major().1 .. corners_crop.upper_right_row_major().1 ,
                0..self.get_color_channel_count()]);
        self.pixels = sliced_array_view.into_owned();
        Ok(self)
    }

    /// Resizes the image using nearest neighbor. Low quality, but fast.
    ///
    /// This method modifies the image in-place by resizing it to the target resolution
    /// using nearest neighbor interpolation. While this is a fast method, it may result
    /// in lower quality compared to other interpolation methods. This operation is not
    /// done in place and instead allocates a new array.
    ///
    /// # Arguments
    ///
    /// * `target_width_height` - The desired resolution as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(&mut Self) if the resize operation was successful
    /// - Err(DataProcessingError) if the target resolution is invalid (zero or negative)
    pub fn resize_nearest_neighbor(&mut self, target_width_height: &(usize, usize)) -> Result<&mut Self, FeagiDataProcessingError> {
        if target_width_height.0 <= 0 || target_width_height.1 <= 0 {
            return Err(IODataError::InvalidParameters("The target resize width or height cannot be zero or negative!".into()).into())
        }
        let source_resolution: (usize, usize) = self.get_internal_resolution();
        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32); // Y X order
        let number_color_channels: usize = self.get_color_channel_count();

        let mut sized_array: Array3<f32> = Array3::zeros((target_width_height.1, target_width_height.0, number_color_channels));
        let target_width_height_f: (f32, f32) = (target_width_height.0 as f32, target_width_height.1 as f32);
        for ((y, x, c), color_val) in sized_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / source_resolution_f.0) * target_width_height_f.1).floor() as usize;
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / source_resolution_f.1) * target_width_height_f.0).floor() as usize;
            let nearest_neighbor_channel_value: f32 = self.pixels[(nearest_neighbor_coordinate_x, nearest_neighbor_coordinate_y, c)];
            *color_val = nearest_neighbor_channel_value;
        };
        self.pixels = sized_array;

        Ok(self)
    }

    // TODO crop into self

    // TODO resize into self

    // TODO crop and resize into self

    // endregion

    // region: Load Data in place
    
    /// Processes a source image frame in-place using the specified processing parameters.
    /// 
    /// This method applies a series of image processing operations to a source frame
    /// and stores the result in this frame. The processing can include cropping,
    /// resizing, brightness adjustment, and contrast adjustment.
    /// 
    /// # Arguments
    /// 
    /// * `image_processing` - The processing parameters defining which operations to apply
    /// * `source` - The source ImageFrame to process
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if all processing steps were successful
    /// - Err(DataProcessingError) if any processing step fails or if the frames are incompatible
    pub fn in_place_run_processor(&mut self, image_processing: FrameProcessingParameters, source: ImageFrame) -> Result<(), FeagiDataProcessingError> {
        
        self.err_if_incoming_image_frame_is_color_incompatible(&source)?;
        
        
        if image_processing.get_final_width_height().is_ok(){
            if image_processing.get_final_width_height()? != self.get_cartesian_width_height() {
                return Err(IODataError::InvalidInplaceOperation("Specified Frame Processing Parameters do not result in an image that can fit in this ImageFrame!".into()).into());
            }
        }
        else { 
            // The image is not being cropped or resized, so it must fit already
            if self.get_internal_resolution() != source.get_internal_resolution() {
                return Err(IODataError::InvalidInplaceOperation("Input image is not being cropped or resized, and does not have the correct dimensions to fit in this ImageFrame!".into()).into());
            }
        }
        
        let processing_steps_required = image_processing.process_steps_required_to_run();

        // bool order is {cropping_from, resizing_to, multiply_brightness, contrast, to_grayscale, color_space}
        match processing_steps_required { // I can't believe this isn't Yandresim!
            (false, false, false, false, false, false) => {
                // No processing steps specified
                self.in_place_load_data_unchanged(source.pixels, ImageFrame::INTERNAL_MEMORY_LAYOUT)?;
                Ok(())
            }

            (true, false, false, false, false, false) => {
                // cropping from only
                self.in_place_crop_image(&image_processing.cropping_from.unwrap(), &source)?;
                Ok(())
            }

            (false, true, false, false, false, false) => {
                // resize only
                self.in_place_nearest_neighbor_resize(&source)?;
                Ok(())
            }

            (true, true, false, false, false, false) => {
                // crop from and resize to
                self.in_place_crop_and_nearest_neighbor_resize(&image_processing.cropping_from.unwrap(), &source)?;
                Ok(())
            }
            

            _ => {
                // We do not have an optimized pathway, just do this sequentially (although this is considerably slower)

                Err(FeagiDataProcessingError::NotImplemented) // TODO
                /*
                
                let mut frame = ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT)?;


                if image_processing.cropping_from.is_some() {
                    let corner_points_cropping = &image_processing.cropping_from.unwrap();
                    let _ = frame.crop_to(corner_points_cropping)?;
                };

                if image_processing.resizing_to.is_some() {
                    let corner_points_resizing = &image_processing.resizing_to.unwrap();
                    let _ = frame.resize_nearest_neighbor(corner_points_resizing)?;
                };

                if image_processing.multiply_brightness_by.is_some() {
                    let brightness = image_processing.multiply_brightness_by.unwrap();
                    let _ = frame.change_brightness_multiplicative(brightness)?;
                };

                if image_processing.change_contrast_by.is_some() {
                    let change_contrast_by = image_processing.change_contrast_by.unwrap();
                    let _ = frame.change_contrast(change_contrast_by)?;
                };

                // TODO grayscale conversions and color space conversions!

                Ok(())
                
                 */
            }
        }
    }
    
    /// Loads pixel data from an array into this frame without any processing.
    /// 
    /// This method replaces the current pixel data with data from the provided array,
    /// converting the memory layout if necessary.
    /// 
    /// # Arguments
    /// 
    /// * `new_array` - The new pixel data to load
    /// * `source_memory_order` - The memory layout of the source array
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the data was loaded successfully
    /// - Err(DataProcessingError) if the array dimensions don't match this frame
    pub fn in_place_load_data_unchanged(&mut self, new_array: Array3<f32>, source_memory_order: MemoryOrderLayout) -> Result<(), FeagiDataProcessingError> {
        if new_array.shape()[2] != self.get_color_channel_count() {
            return Err(IODataError::InvalidParameters("Input array does not seem to have the correct number of color channels!".into()).into());
        }
        if (new_array.shape()[0] , new_array.shape()[1]) != self.get_internal_resolution() {
            return Err(IODataError::InvalidParameters("Input array does not seem to have the correct height or width!".into()).into());
        }
        self.pixels = ImageFrame::change_memory_order_to_row_major(new_array.into_owned(), source_memory_order);
        Ok(())
    }
    
    /// Crops a region from a source image and stores it in this frame.
    /// 
    /// This method extracts a rectangular region from the source image as defined
    /// by the cropping points and stores it in this frame.
    /// 
    /// # Arguments
    /// 
    /// * `source_cropping_points` - The region to crop from the source image
    /// * `source` - The source ImageFrame to crop from
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the crop operation was successful
    /// - Err(DataProcessingError) if the frames are incompatible or crop region is invalid
    pub fn in_place_crop_image(&mut self, source_cropping_points: &CornerPoints, source: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        if source.get_channel_layout() != self.get_channel_layout() {
            return Err(IODataError::InvalidParameters("The given image does not have the same color channel count as this ImageFrame!".into()).into())
        }

        if source.get_color_space() != self.get_color_space() {
            return Err(IODataError::InvalidParameters("The given image does not have the same color space as this ImageFrame!".into()).into())
        }
        
        if !source_cropping_points.does_fit_in_frame_of_width_height(source.get_cartesian_width_height()) {
            return Err(IODataError::InvalidParameters("The given cropped region exceeds the source boundaries!".into()).into())
        }
        
        if source_cropping_points.enclosed_area_width_height() != self.get_cartesian_width_height() {
            return Err(IODataError::InvalidParameters("The given cropped region does not have the same area as this image!".into()).into())
        }

        let channel_count: usize = source.get_color_channel_count();
        let sliced_array_view: ArrayView3<f32> = source.pixels.slice(
            s![source_cropping_points.lower_left_row_major().0 .. source_cropping_points.upper_right_row_major().0,
                source_cropping_points.lower_left_row_major().1 .. source_cropping_points.upper_right_row_major().1 , 0..channel_count]
        );
        self.pixels = sliced_array_view.into_owned();
        Ok(())
    }
    
    /// Resizes a source image to fit this frame using nearest neighbor interpolation.
    /// 
    /// This method resizes the source image to match the dimensions of this frame
    /// using nearest neighbor interpolation for speed.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The source ImageFrame to resize
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the resize operation was successful
    /// - Err(DataProcessingError) if the frames are incompatible
    pub fn in_place_nearest_neighbor_resize(&mut self, source: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        // We don't need to specify size, as we are just using this size
        if source.get_channel_layout() != self.get_channel_layout() {
            return Err(IODataError::InvalidParameters("The given image does not have the same color channel count as this ImageFrame!".into()).into())
        }

        if source.get_color_space() != self.get_color_space() {
            return Err(IODataError::InvalidParameters("The given image does not have the same color space as this ImageFrame!".into()).into())
        }

        let resolution: (usize, usize) = self.get_internal_resolution();
        let source_resolution = source.get_internal_resolution();
        let resolution_f: (f32, f32) = (resolution.0 as f32, resolution.1 as f32);
        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32);

        for ((y,x,c), color_val) in self.pixels.indexed_iter_mut() {
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / resolution_f.1) * source_resolution_f.1).floor() as usize;
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / resolution_f.0) * source_resolution_f.0).floor() as usize;
            let nearest_neighbor_channel_value: f32 = source.pixels[(
                nearest_neighbor_coordinate_x,
                nearest_neighbor_coordinate_y,
                c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(())
    }
    
    /// Crops and resizes a source image in a single operation.
    /// 
    /// This method first crops the specified region from the source image, then
    /// resizes the cropped region to fit this frame using nearest neighbor interpolation.
    /// This is more efficient than performing crop and resize as separate operations.
    /// 
    /// # Arguments
    /// 
    /// * `source_cropping_points` - The region to crop from the source image
    /// * `source` - The source ImageFrame to process
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(DataProcessingError) if the frames are incompatible or crop region is invalid
    pub fn in_place_crop_and_nearest_neighbor_resize(&mut self, source_cropping_points: &CornerPoints, source: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        
        let crop_resolution: (usize, usize) = source_cropping_points.enclosed_area_width_height();
        if !ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(&self, source) {
            return Err(IODataError::InvalidInplaceOperation("The incoming source data does not have compatible properties with the destination ImageFrame!".into()).into())
        }

        let resolution: (usize, usize) = self.get_internal_resolution();
        let resolution_f: (f32, f32) = (resolution.0 as f32, resolution.1 as f32);
        let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);
        
        let dist_factor_yx: (f32, f32) = (
            (crop_resolution_f.1 / resolution_f.1),
            (crop_resolution_f.0 / resolution_f.0));
        
        let upper_left_corner_offset_yx: (usize, usize) = (
            source_cropping_points.upper_left_row_major().0,
            source_cropping_points.upper_left_row_major().1,
            );
        
        for ((y,x,c), color_val) in self.pixels.indexed_iter_mut() {
            let nearest_neighbor_coordinate_from_source_y: usize = ((y as f32) * dist_factor_yx.0).floor() as usize;
            let nearest_neighbor_coordinate_from_source_x: usize = ((x as f32) * dist_factor_yx.1).floor() as usize;
            let nnc_y_with_offset = nearest_neighbor_coordinate_from_source_y + upper_left_corner_offset_yx.0;
            let nnc_x_with_offset = nearest_neighbor_coordinate_from_source_x + upper_left_corner_offset_yx.1;
            
            let nearest_neighbor_channel_value: f32 = source.pixels[(
                nnc_y_with_offset,
                nnc_x_with_offset,
                c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(())
    }

    /// Calculates the thresholded difference between two frames and stores the result.
    /// 
    /// This method computes the absolute difference between corresponding pixels in
    /// two frames. If the difference exceeds the threshold, the difference value is
    /// stored; otherwise, zero is stored.
    /// 
    /// # Arguments
    /// 
    /// * `previous_frame` - The first frame for comparison
    /// * `next_frame` - The second frame for comparison
    /// * `threshold` - The minimum difference required to register a change
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(DataProcessingError) if the frames have incompatible dimensions
    pub fn in_place_calculate_difference_thresholded(&mut self, previous_frame: &ImageFrame, next_frame: &ImageFrame, threshold: u8) -> Result<(), FeagiDataProcessingError> {
        if !ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(&previous_frame, next_frame) {
            return Err(IODataError::InvalidInplaceOperation("The two given frames do not have equivalent resolutions or channel counts!".into()).into())
        }
        if !ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(self, next_frame) {
            return Err(IODataError::InvalidInplaceOperation("This frame does not have equivalent resolutions or channel count to the given comparing frames!".into()).into())
        }
        let threshold: f32 = threshold as f32; // TODO will we be changing the internal data structure from float? probably not due to some conversions being float based
        for (coord, color_val) in self.pixels.indexed_iter_mut() {
            let prev_frame_val: f32 = previous_frame.pixels[coord];
            let next_frame_val: f32 = next_frame.pixels[coord];
            let delta: f32 = (prev_frame_val - next_frame_val).abs();
            if delta > threshold{
                *color_val = delta;
            }
            else {
                *color_val = 0.0;
            }
        };
        Ok(())
    }

    // endregion

    // region: neuron export

    /// Converts pixel data to neuron arrays using a threshold filter.
    /// 
    /// This method extracts pixels with values above the specified threshold and
    /// converts them to neuron data with X, Y, channel, and potential values.
    /// The Y coordinates are flipped to convert from image coordinates to FEAGI's
    /// Cartesian coordinate system.
    /// 
    /// # Arguments
    /// 
    /// * `threshold` - The minimum pixel value required to generate a neuron
    /// * `write_target` - The NeuronXYCPArrays to write the neuron data to
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the conversion was successful
    /// - Err(DataProcessingError) if the operation fails
    pub fn write_xyzp_neuron_arrays(& self, write_target: &mut NeuronXYZPArrays, x_channel_offset: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {
        const EPSILON: f32 = 0.0001; // avoid writing near zero vals
        
        let y_flip_distance: u32 = self.get_internal_shape().0 as u32;
        write_target.expand_to_new_max_count_if_required(self.get_max_capacity_neuron_count()); // make sure there's enough capacity
        write_target.reset_indexes(); // Ensure we push from the start
        let x_offset: u32 = *x_channel_offset * self.get_cartesian_width_height().0 as u32;
        
        // write to the vectors
        write_target.update_vectors_from_external(|x_vec, y_vec, c_vec, p_vec| {
            for ((y, x, c), color_val) in self.pixels.indexed_iter() { // going from row major to cartesian
                if color_val.abs() > EPSILON {
                    x_vec.push(x as u32 + x_offset);
                    y_vec.push( y_flip_distance - y as u32);  // flip y
                    c_vec.push(c as u32);
                    p_vec.push(*color_val);
                }
            };
            Ok(())
        })
    }
    // endregion

    // region: specialized constructors
    // These are called from the FrameProcessingParameters constructors

    /// Creates a new ImageFrame by cropping a region from a source frame, followed by a resize
    /// to the given resolution.
    ///
    /// This function first crops the specified region from the source frame, then resizes
    /// the cropped region to the target resolution using nearest neighbor interpolation.
    /// This function assumes HeightsWidthsChannels ordering.
    ///
    /// # Arguments
    ///
    /// * `source_frame` - The source ImageFrame to crop from
    /// * `corners_crop` - The CornerPoints defining the region to crop
    /// * `new_width_height` - The target resolution as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if the crop region is valid and fits within the source frame
    /// - Err(DataProcessingError) if the crop region would not fit in the source frame
    ///
    /// ```
    pub fn create_from_source_frame_crop_and_resize(source_frame: &ImageFrame, corners_crop: &CornerPoints, new_width_height: &(usize, usize)) -> Result<ImageFrame, FeagiDataProcessingError> {
        let source_resolution = source_frame.get_internal_resolution(); // Y X
        if !corners_crop.does_fit_in_frame_of_width_height(source_resolution) {
            return Err(IODataError::InvalidParameters("The given crop would not fit in the given source!".into()).into())
        }

        let pixels_source = source_frame.get_pixels_view();
        let crop_resolution_f: (f32, f32) = (new_width_height.1 as f32, new_width_height.0 as f32); // Y X
        let new_resolution_f: (f32, f32) = (new_width_height.1 as f32, new_width_height.0 as f32); // Y X
        let lower_left_offset = corners_crop.lower_left_row_major(); // Y X
        let mut writing_array: Array3<f32> = Array3::<f32>::zeros((new_width_height.0, new_width_height.1, source_frame.get_color_channel_count()));

        for ((y, x, c), color_val) in writing_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_y: usize = lower_left_offset.0 - (((y as f32) / crop_resolution_f.0) * new_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_x: usize = lower_left_offset.1 + (((x as f32) / crop_resolution_f.1) * new_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = pixels_source[(nearest_neighbor_coordinate_y, nearest_neighbor_coordinate_x, c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(ImageFrame {
            pixels: writing_array,
            channel_layout: source_frame.channel_layout,
            color_space: source_frame.color_space,
        })
    }

    /// Creates a new ImageFrame by cropping a region from a source frame.
    /// 
    /// This function extracts a rectangular region from the source frame as defined
    /// by the corner points and creates a new ImageFrame containing only that region.
    /// 
    /// # Arguments
    /// 
    /// * `source_frame` - The source ImageFrame to crop from
    /// * `corners_crop` - The CornerPoints defining the region to crop
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(ImageFrame) if the crop region is valid and fits within the source frame
    /// - Err(DataProcessingError) if the crop region would not fit in the source frame
    pub fn create_from_source_frame_crop(source_frame: &ImageFrame, corners_crop: &CornerPoints) -> Result<ImageFrame, FeagiDataProcessingError> {
        let source_resolution = source_frame.get_internal_resolution(); // TODO ?
        if !corners_crop.does_fit_in_frame_of_width_height(source_resolution) {
            return Err(IODataError::InvalidParameters("The given crop would not fit in the given source!".into()).into())
        }

        let channel_count: usize = source_frame.get_color_channel_count();
        let sliced_array_view: ArrayView3<f32> = source_frame.pixels.slice(
            s![corners_crop.lower_left_row_major().0 .. corners_crop.upper_right_row_major().0,
                corners_crop.lower_left_row_major().1 .. corners_crop.upper_right_row_major().1 , 0..channel_count]
        );
        Ok(ImageFrame {
            pixels: sliced_array_view.into_owned(),
            channel_layout: source_frame.channel_layout,
            color_space: source_frame.color_space,
        })
    }

    // endregion



    // region: internal functions
    
    /// Converts an array from any memory layout to row-major (HeightsWidthsChannels) format.
    ///
    /// This internal function handles the conversion of arrays between different memory
    /// layouts, ensuring they are stored in the standard row-major format used internally.
    ///
    /// # Arguments
    ///
    /// * `input` - The input array to convert
    /// * `source_memory_order` - The current memory layout of the input array
    ///
    /// # Returns
    ///
    /// The converted array in row-major format.
    fn change_memory_order_to_row_major(input: Array3<f32>, source_memory_order: MemoryOrderLayout) -> Array3<f32> {
        match source_memory_order {
            MemoryOrderLayout::HeightsWidthsChannels => input, // Nothing needed, we store in this format anyway
            MemoryOrderLayout::ChannelsHeightsWidths => input.permuted_axes([2, 0, 1]),
            MemoryOrderLayout::WidthsHeightsChannels => input.permuted_axes([1, 0, 2]),
            MemoryOrderLayout::HeightsChannelsWidths => input.permuted_axes([0, 2, 1]),
            MemoryOrderLayout::ChannelsWidthsHeights => input.permuted_axes([2, 1, 0]),
            MemoryOrderLayout::WidthsChannelsHeights => input.permuted_axes([1, 2, 0]),
        }
    }

    /// Converts an array from row-major format to any other memory layout.
    ///
    /// This internal function handles the conversion of arrays from the internal
    /// row-major format to any other specified memory layout.
    ///
    /// # Arguments
    ///
    /// * `input` - The input array in row-major format
    /// * `target_memory_order` - The desired memory layout for the output array
    ///
    /// # Returns
    ///
    /// The converted array in the target memory layout.
    fn change_memory_order_from_row_major(input: Array3<f32>, target_memory_order: MemoryOrderLayout) -> Array3<f32> {
        match target_memory_order {
            MemoryOrderLayout::HeightsWidthsChannels => input, // Nothing needed, we store in this format anyway
            MemoryOrderLayout::ChannelsHeightsWidths => input.permuted_axes([1, 2, 0]),
            MemoryOrderLayout::WidthsHeightsChannels => input.permuted_axes([1, 0, 2]),
            MemoryOrderLayout::HeightsChannelsWidths => input.permuted_axes([0, 2, 1]),
            MemoryOrderLayout::ChannelsWidthsHeights => input.permuted_axes([2, 1, 0]),
            MemoryOrderLayout::WidthsChannelsHeights => input.permuted_axes([2, 0, 1]),
        }
    }
    
    /// Validates that an incoming ImageFrame has compatible color properties.
    /// 
    /// This internal function checks that the incoming frame has the same color space
    /// and channel format as this frame, which is required for in-place operations.
    /// 
    /// # Arguments
    /// 
    /// * `incoming` - The ImageFrame to validate for compatibility
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the frames are compatible
    /// - Err(DataProcessingError) if the color properties don't match
    fn err_if_incoming_image_frame_is_color_incompatible(&self, incoming: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        if self.color_space != incoming.color_space {
            return Err(IODataError::InvalidParameters("Incoming source array does not have matching color space!".into()).into())
        }
        if self.channel_layout == incoming.channel_layout {
            return Err(IODataError::InvalidParameters("Incoming source array does not have matching color channel count!!".into()).into())
        }
        Ok(())
    }
    
    // endregion

}
