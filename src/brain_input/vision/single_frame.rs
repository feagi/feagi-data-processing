use ndarray::{s, Array3, ArrayView3};
use crate::Error::DataProcessingError;
use super::single_frame_processing::*;


/// A structure representing an image frame with pixel data and channel format information.
///
/// Various functions exist for processing images for use with FEAGI
/// Internally, uses a 3D ndarray of f32 values from 0-1, stored in
/// row major ordering (Heights Widths Channels)
///
/// # Examples
///
/// ```
/// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
/// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
///
/// // Create a new RGB image frame
/// let resolution = (640, 480);
/// let row_major_resolution = (480, 640);
/// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &resolution);
///
/// // Get the image resolution
/// assert_eq!(frame.get_cartesian_width_height(), row_major_resolution);
/// ```
#[derive(Clone)]
pub struct ImageFrame {
    pixels: Array3<f32>,
    channel_format: ChannelFormat,
    color_space: ColorSpace,
}

impl ImageFrame {
    
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
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(640, 480));
    /// let row_major_resolution = (480, 640);
    /// assert_eq!(frame.get_cartesian_width_height(), row_major_resolution);
    /// assert_eq!(frame.get_color_channel_count(), 3);
    /// ```
    pub fn new(channel_format: &ChannelFormat, color_space: &ColorSpace, xy_resolution: &(usize, usize)) -> ImageFrame {
        ImageFrame {
            channel_format: *channel_format,
            color_space: *color_space,
            pixels: Array3::<f32>::zeros((xy_resolution.1, xy_resolution.0, *channel_format as usize)),
        }
    }

    /// Creates an ImageFrame from an existing ndarray with the specified color space.
    ///
    /// # Arguments
    ///
    /// * `color_space` - The color space of the image (Linear or Gamma)
    /// * `input` - A 3D array of f32 values representing the image pixels
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if the input array has a valid number of color channels (1-4)
    /// - Err(DataProcessingError) if the number of color channels is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let array = Array3::<f32>::zeros((100, 100, 3)); // RGB image
    /// let frame: ImageFrame = ImageFrame::from_array(array, ColorSpace::Gamma, MemoryOrderLayout::HeightsWidthsChannels).unwrap();
    /// assert_eq!(frame.get_color_channel_count(), 3);
    /// ```
    pub fn from_array(input: Array3<f32>, color_space: ColorSpace, source_memory_order: MemoryOrderLayout) -> Result<ImageFrame, DataProcessingError> {
        let number_color_channels: usize = input.shape()[2];
        Ok(ImageFrame {
            pixels: ImageFrame::change_memory_order_to_row_major(input, source_memory_order),
            color_space,
            channel_format: usize_to_channel_format(number_color_channels)?
        })
    }
    
    /// Creates an ImageFrame from an existing ndarray with optional processing steps.
    ///
    /// This function allows creating an ImageFrame with a series of optional processing steps
    /// such as cropping, resizing, brightness adjustment, and contrast adjustment.
    ///
    /// # Arguments
    ///
    /// * `color_space` - The color space of the image (Linear or Gamma)
    /// * `image_processing` - Parameters defining the processing steps to apply
    /// * `input` - A 3D array of f32 values representing the image pixels
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if all processing steps were successful
    /// - Err(DataProcessingError) if any processing step fails
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let array = Array3::<f32>::zeros((100, 100, 3));
    /// let mut params = FrameProcessingParameters::new();
    /// params.set_multiply_brightness_by(1.5).unwrap().set_change_contrast_by(0.5).unwrap();
    /// let frame = ImageFrame::from_array_with_processing(ColorSpace::Gamma, params, array).unwrap();
    /// ```
    pub fn from_array_with_processing(source_color_space: ColorSpace, image_processing: FrameProcessingParameters, input: Array3<f32>) -> Result<ImageFrame, DataProcessingError> {
        // Lets set the memory order correct first, this has 0 cost
        let processed_input = ImageFrame::change_memory_order_to_row_major(input, image_processing.get_memory_ordering_of_source());
        
        let processing_steps_required = image_processing.process_steps_required_to_run();
        
        // there are 6! (720) permutations of these bools. I ain't writing them all out here. Let us stick to the most common ones
        // bool order is cropping_from, resizing_to, multiply_brightness, contrast, to_grayscale, color_space
        match processing_steps_required { // I can't believe this isn't Yandresim!
            (false, false, false, false, false, false) => {
                // No processing steps specified
                return ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT)
            }

            (true, true, false, false, false, false) => {
                // crop from and resize to
                let source_frame = ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT);
                return ImageFrame::create_from_source_array_crop_and_resize(&source_frame?, &image_processing.get_cropping_from().unwrap(), &image_processing.get_resizing_to().unwrap());
            }
            
            
            _ => {
                // We do not have an optimized pathway, just do this sequentially (although this is considerably slower)
                let mut frame = ImageFrame::from_array(processed_input, source_color_space, ImageFrame::INTERNAL_MEMORY_LAYOUT)?;
                
                if image_processing.get_cropping_from().is_some(){
                    let corner_points_cropping = &image_processing.get_cropping_from().unwrap();
                    let _ = frame.crop_to(corner_points_cropping)?;
                };
                
                if image_processing.get_resizing_to().is_some(){
                    let corner_points_resizing = &image_processing.get_resizing_to().unwrap();
                    let _ = frame.resize_nearest_neighbor(corner_points_resizing)?;
                };
                
                if image_processing.get_multiply_brightness_by().is_some(){
                    let brightness = image_processing.get_multiply_brightness_by().unwrap();
                    let _ = frame.change_brightness_multiplicative(brightness)?;
                };
                
                if image_processing.get_change_contrast_by().is_some(){
                    let change_contrast_by = image_processing.get_change_contrast_by().unwrap();
                    let _ = frame.change_contrast(change_contrast_by)?;
                };
                
                // TODO grayscale conversions and color space conversions!
                
                return Ok(frame);
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
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame1 = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let frame2 = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// assert!(ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(&frame1, &frame2));
    /// ```
    pub fn do_resolutions_channel_depth_and_color_spaces_match(a: &ImageFrame, b: &ImageFrame) -> bool {
        a.get_internal_shape() == b.get_internal_shape() && a.color_space == b.color_space
    }
    
    pub fn is_array_valid_for_image_frame(array: &Array3<f32>) -> bool {
        let shape: &[usize] =  array.shape();
        if shape[2] > 4 || shape[2] == 0{
            return false;
        }
        if shape[0] == 0 || shape[1] == 0{
            return false;
        }
        true
    }
    
    /// Returns a reference to the channel format of this image.
    ///
    /// # Returns
    ///
    /// A reference to the ChannelFormat enum value representing the image's color channel format.
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// assert_eq!(*frame.get_channel_format(), ChannelFormat::RGB);
    /// ```
    pub fn get_channel_format(&self) -> &ChannelFormat {
        &self.channel_format
    }

    /// Returns a reference to the color space of this image.
    ///
    /// # Returns
    ///
    /// A reference to the ColorSpace enum value representing the image's color space.
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// assert_eq!(*frame.get_color_space(), ColorSpace::Gamma);
    /// ```
    pub fn get_color_space(&self) -> &ColorSpace {
        &self.color_space
    }
    
    /// Returns the number of color channels in this ImageFrame.
    ///
    /// # Returns
    ///
    /// The number of color channels as a usize:
    /// - 1 for GrayScale
    /// - 2 for RG
    /// - 3 for RGB
    /// - 4 for RGBA
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// assert_eq!(frame.get_color_channel_count(), 3);
    /// ```
    pub fn get_color_channel_count(&self) -> usize {
        self.channel_format as usize
    }

    /// Returns a read-only view of the pixel data.
    ///
    /// This provides access to the underlying 3D ndarray of pixel values.
    ///
    /// # Returns
    ///
    /// An ArrayView3<f32> containing the pixel data.
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let view = frame.get_pixels_view();
    /// assert_eq!(view.shape(), [100, 100, 3]);
    /// ```
    pub fn get_pixels_view(&self) -> ArrayView3<f32> {
        self.pixels.view()
    }

    /// Returns the resolution of the image in cartesian space (width, height)
    ///
    /// # Returns
    ///
    /// A tuple of (width, height) representing the image dimensions in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(640, 480));
    /// assert_eq!(frame.get_cartesian_width_height(), (640, 480));
    /// ```
    pub fn get_cartesian_width_height(&self) -> (usize, usize) {
        let shape: &[usize] =  self.pixels.shape();
        (shape[1], shape[0]) // because nd array is row major, where coords are yx
    }
    
    pub fn get_internal_resolution(&self) -> (usize, usize) {
        let shape: &[usize] =  self.pixels.shape();
        (shape[0], shape[1])
    }
    
    
    pub fn get_internal_shape(&self) -> (usize, usize, usize) {
        let shape: &[usize] =  self.pixels.shape();
        (shape[0], shape[1], shape[2])
    }
    
    /// Calculates the number of bytes needed to store the XYZP data.
    ///
    /// Each voxel (pixel) requires 16 bytes of storage:
    /// - 4 bytes for X coordinate (u32)
    /// - 4 bytes for Y coordinate (u32)
    /// - 4 bytes for Z coordinate (u32)
    /// - 4 bytes for potential value (f32)
    ///
    /// # Returns
    ///
    /// The total number of bytes needed to store all voxel data.
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let bytes_needed = frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
    /// assert_eq!(bytes_needed, 100 * 100 * 3 * 16); // width * height * channels * bytes_per_voxel
    /// ```
    pub fn get_number_of_bytes_needed_to_hold_xyzp_uncompressed(& self) -> usize {
        const NUMBER_BYTES_PER_VOXEL: usize = 16;
        let dimensions = self.pixels.shape(); // we know its 3 elements
        dimensions[0] * dimensions[1] * dimensions[2] * NUMBER_BYTES_PER_VOXEL
    }
    
    // endregion
    
    // region: mutate structure (in place, no memory reallocations)

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
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// frame.change_brightness_multiplicative(1.5).unwrap(); // Increase brightness by 50%
    /// ```
    pub fn change_brightness_multiplicative(&mut self, brightness_factor: f32) -> Result<(), DataProcessingError> {
        if brightness_factor < 0.0 {
            return Err(DataProcessingError::InvalidInputBounds("Multiply brightness by must be positive!".into()));
        }

        self.pixels.mapv_inplace(|v| {
            let scaled = (v as f32) * brightness_factor;
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
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// frame.change_contrast(0.5).unwrap();  // Increase contrast
    /// frame.change_contrast(-0.3).unwrap(); // Decrease contrast
    /// ```
    pub fn change_contrast(&mut self, contrast_factor: f32) -> Result<(), DataProcessingError> {
        if contrast_factor < -1.0 || contrast_factor > 1.0 {
            return Err(DataProcessingError::InvalidInputBounds("The contrast factor must be between -1.0 and 1.0!".into()));
        }
        // Algo sourced from https://ie.nitk.ac.in/blog/2020/01/19/algorithms-for-adjusting-brightness-and-contrast-of-an-image/
        const CORRECTION_FACTOR: f32 = 1.015686; //  259 / 255
        self.pixels.mapv_inplace(|v| {
            let factor: f32 =  (CORRECTION_FACTOR * (contrast_factor + 1.0)) / (CORRECTION_FACTOR - contrast_factor);
            let pixel_val: f32 = (factor * (v - 0.5)) + 0.5;
            pixel_val.clamp(0.0, 1.0)

        });
        Ok(())
    }
    
    // TODO Color Space Transformations
    
    // endregion

    // region: mutate structure (non-in-place)
    
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
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let image_original_resolution = (100, 100);
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &image_original_resolution);
    /// let corners = CornerPoints::new_from_cartesian_where_origin_bottom_left((10, 10), (50, 50), image_original_resolution).unwrap();
    /// frame.crop_to(&corners).unwrap();
    /// assert_eq!(frame.get_cartesian_width_height(), (40, 40));
    /// ```
    pub fn crop_to(&mut self, corners_crop: &CornerPoints) -> Result<&mut Self, DataProcessingError> {
        if !corners_crop.does_fit_in_frame_of_resolution(self.get_cartesian_width_height()) {
            return Err(DataProcessingError::InvalidInputBounds("The given crop would not fit in the given source!".into()));
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
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// frame.resize_nearest_neighbor(&(50, 50)).unwrap();
    /// assert_eq!(frame.get_cartesian_width_height(), (50, 50));
    /// ```
    pub fn resize_nearest_neighbor(&mut self, target_width_height: &(usize, usize)) -> Result<&mut Self, DataProcessingError> {
        if target_width_height.0 <= 0 || target_width_height.1 <= 0 {
            return Err(DataProcessingError::InvalidInputBounds("The target resize width or height cannot be zero or negative!".into()))
        }
        let source_resolution: (usize, usize) = self.get_internal_resolution();
        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32); // Y X order
        let number_color_channels: usize = self.get_color_channel_count();

        let mut sized_array: Array3<f32> = Array3::zeros((target_width_height.1, target_width_height.0, number_color_channels));
        let target_width_height_f: (f32, f32) = (target_width_height.0 as f32, target_width_height.1 as f32);
        for ((y,x,c), color_val) in sized_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / source_resolution_f.0) * target_width_height_f.1).floor() as usize;
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / source_resolution_f.1) * target_width_height_f.0).floor() as usize;
            let nearest_neighbor_channel_value: f32 = self.pixels[(nearest_neighbor_coordinate_x, nearest_neighbor_coordinate_y, c)];
            *color_val = nearest_neighbor_channel_value;
        };
        self.pixels = sized_array;
        Ok(self)
    }
    
    // TODO to grayscale
    
    // endregion
    
    // region: byte data

    /// Converts the image frame into a byte array containing XYZP data.
    ///
    /// The output array contains interleaved XYZP data for each voxel:
    /// - X coordinates (u32) for all voxels
    /// - Y coordinates (u32) for all voxels
    /// - Z coordinates (u32) for all voxels
    /// - Potential values (f32) for all voxels
    ///
    /// # Returns
    ///
    /// A Vec<u8> containing the serialized XYZP data.
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let bytes = frame.to_bytes();
    /// assert_eq!(bytes.len(), frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed());
    /// ```
    pub fn to_bytes(& self) -> Vec<u8> {
        let required_number_elements = self.get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
        let mut output: Vec<u8> = Vec::with_capacity(required_number_elements);
        output.resize(required_number_elements, 0x00);

        let mut x_offset: usize = 0;
        let mut y_offset: usize = required_number_elements / 4;
        let mut c_offset: usize = y_offset * 2;
        let mut p_offset: usize = y_offset * 3;

        for ((x,y,c), color_val) in self.pixels.indexed_iter() {
            let x_bytes: [u8; 4] = (x as u32).to_le_bytes();
            let y_bytes: [u8; 4] = (y as u32).to_le_bytes();
            let c_bytes: [u8; 4] = (c as u32).to_le_bytes();
            let p_bytes: [u8; 4] = color_val.to_le_bytes();

            output[x_offset .. x_offset + 4].copy_from_slice(&x_bytes);
            output[y_offset .. y_offset + 4].copy_from_slice(&y_bytes);
            output[c_offset .. c_offset + 4].copy_from_slice(&c_bytes);
            output[p_offset .. p_offset + 4].copy_from_slice(&p_bytes);
            x_offset += 4;
            y_offset += 4;
            c_offset += 4;
            p_offset += 4;
        };
        output
    }

    /// Writes the image frame's XYZP data into a provided byte buffer.
    ///
    /// This is an in-place version of `to_bytes()` that writes directly into
    /// a pre-allocated buffer instead of creating a new Vec.
    ///
    /// # Arguments
    ///
    /// * `bytes_writing_to` - A mutable slice where the XYZP data will be written
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(DataProcessingError) if the provided buffer is too small
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let mut buffer = vec![0u8; frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed()];
    /// frame.to_bytes_in_place(&mut buffer).unwrap();
    /// ```
    pub fn to_bytes_in_place(& self, bytes_writing_to: &mut [u8]) -> Result<(), DataProcessingError> {
        let required_capacity: usize = self.get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
        if bytes_writing_to.len() < required_capacity {
            return Err(DataProcessingError::InvalidInputBounds("Given byte buffer is too small!".into()))
        };

        let mut x_offset: usize = 0;
        let mut y_offset: usize = required_capacity / 4;
        let mut c_offset: usize = y_offset * 2;
        let mut p_offset: usize = y_offset * 3;

        for ((x,y,c), color_val) in self.pixels.indexed_iter() {
            let x_bytes: [u8; 4] = (x as u32).to_le_bytes();
            let y_bytes: [u8; 4] = (y as u32).to_le_bytes();
            let c_bytes: [u8; 4] = (c as u32).to_le_bytes();
            let p_bytes: [u8; 4] = color_val.to_le_bytes();

            bytes_writing_to[x_offset .. x_offset + 4].copy_from_slice(&x_bytes);
            bytes_writing_to[y_offset .. y_offset + 4].copy_from_slice(&y_bytes);
            bytes_writing_to[c_offset .. c_offset + 4].copy_from_slice(&c_bytes);
            bytes_writing_to[p_offset .. p_offset + 4].copy_from_slice(&p_bytes);
            x_offset += 4;
            y_offset += 4;
            c_offset += 4;
            p_offset += 4;
        };
        Ok(())
    }
    
    // endregion

    // region: specialized constructors
    // These are called from the FrameProcessingParameters constructors

    /// Creates a new ImageFrame by cropping a region from a source frame, followed by a resize
    /// to the given resolution
    ///
    /// This function first crops the specified region from the source frame, then resizes
    /// the cropped region to the target resolution using nearest neighbor interpolation.
    /// This function assumes HeightsWidthsChannels ordering!
    ///
    /// # Arguments
    ///
    /// * `source_frame` - The source ImageFrame to crop from
    /// * `corners_crop` - The CornerPoints defining the region to crop
    /// * `new_width_height` - The target cartesian resolution as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if the crop region is valid and fits within the source frame
    /// - Err(&'static str) if the crop region would not fit in the source frame
    ///
    /// ```
    pub fn create_from_source_array_crop_and_resize(source_frame: &ImageFrame, corners_crop: &CornerPoints, new_width_height: &(usize, usize)) -> Result<ImageFrame, DataProcessingError> {
        
        let source_resolution = source_frame.get_internal_resolution(); // Y X
        if !corners_crop.does_fit_in_frame_of_resolution(source_resolution) {
            return Err(DataProcessingError::InvalidInputBounds("The given crop would not fit in the given source!".into()))
        }
        
        let pixels_source = source_frame.get_pixels_view();
        let crop_resolution_f: (f32, f32) = (new_width_height.1 as f32, new_width_height.0 as f32); // Y X
        let new_resolution_f: (f32, f32) = (new_width_height.1 as f32, new_width_height.0 as f32); // Y X
        let lower_left_offset = corners_crop.lower_left_row_major(); // Y X
        let mut writing_array: Array3<f32> = Array3::<f32>::zeros((new_width_height.0, new_width_height.1, source_frame.get_color_channel_count()));

        for ((y,x,c), color_val) in writing_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_y: usize = lower_left_offset.0 - (((y as f32) / crop_resolution_f.0) * new_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_x: usize = lower_left_offset.1 + (((x as f32) / crop_resolution_f.1) * new_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = pixels_source[(nearest_neighbor_coordinate_y, nearest_neighbor_coordinate_x, c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(ImageFrame {
            pixels: writing_array,
            channel_format: source_frame.channel_format,
            color_space: source_frame.color_space,
        })
    }
    
    
    
    // endregion


    // region: internal functions
    
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
    
    // endregion
    
    









    /*
    
    
        /// Creates a new ImageFrame by cropping a region from a source frame.
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
    /// - Err(&'static str) if the crop region would not fit in the source frame
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let source = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let corners = CornerPoints::new((10, 10), (50, 50)).unwrap();
    /// let cropped = ImageFrame::from_source_frame_crop(&source, &corners).unwrap();
    /// ```
    pub fn from_source_frame_crop(source_frame: &ImageFrame, corners_crop: &CornerPoints) -> Result<ImageFrame, DataProcessingError> {
        let source_resolution = source_frame.get_xy_resolution();
        if !corners_crop.does_fit_in_frame_of_resolution(source_resolution) {
            return Err(DataProcessingError::InvalidInputBounds("The given crop would not fit in the given source!".into()))
        }
        let channel_count: usize = source_frame.get_color_channel_count();
        let sliced_array_view: ArrayView3<f32> = source_frame.pixels.slice(s![corners_crop.lower_left().0 .. corners_crop.upper_right().0, corners_crop.lower_left().1 .. corners_crop.upper_right().1 , 0..channel_count]);
        Ok(ImageFrame {
            pixels: sliced_array_view.into_owned(),
            channel_format: source_frame.channel_format.clone()
        })
    }

    /// Creates a new ImageFrame by cropping a region from a source frame, followed by a resize
    /// to the given resolution
    ///
    /// This function first crops the specified region from the source frame, then resizes
    /// the cropped region to the target resolution using nearest neighbor interpolation.
    ///
    /// # Arguments
    ///
    /// * `source_frame` - The source ImageFrame to crop from
    /// * `corners_crop` - The CornerPoints defining the region to crop
    /// * `new_resolution` - The target resolution as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if the crop region is valid and fits within the source frame
    /// - Err(&'static str) if the crop region would not fit in the source frame
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let source = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let corners = CornerPoints::new((10, 10), (50, 50)).unwrap();
    /// let resized = ImageFrame::from_source_frame_crop_and_resize(&source, &corners, &(200, 200)).unwrap();
    /// ```
    pub fn from_source_frame_crop_and_resize(source_frame: &ImageFrame, corners_crop: &CornerPoints, new_resolution: &(usize, usize)) -> Result<ImageFrame, DataProcessingError> {
        let source_resolution = source_frame.get_xy_resolution();
        if !corners_crop.does_fit_in_frame_of_resolution(source_resolution) {
            return Err(DataProcessingError::InvalidInputBounds("The given crop would not fit in the given source!".into()))
        }
        let channel_count: usize = source_frame.get_color_channel_count();

        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32);
        let crop_resolution_f: (f32, f32) = (new_resolution.0 as f32, new_resolution.1 as f32);
        let mut writing_array: Array3<f32> = Array3::<f32>::zeros((new_resolution.0, new_resolution.1, channel_count));

        for ((x,y,c), color_val) in writing_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / source_resolution_f.0) * crop_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / source_resolution_f.1) * crop_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = source_frame.pixels[(nearest_neighbor_coordinate_x + corners_crop.lower_left().0, nearest_neighbor_coordinate_y + corners_crop.lower_right().1, c)]; // TODO ???
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(ImageFrame {
            pixels: writing_array,
            channel_format: source_frame.channel_format.clone()
        })
    }
    


    /// Crops and resizes a region from a source frame directly into this frame.
    ///
    /// This method modifies this frame in-place by first cropping, then scaling the crop to fit
    /// into this image frame.
    /// The operation uses nearest neighbor interpolation for resizing.
    ///
    /// # Arguments
    ///
    /// * `source_cropping_points` - The CornerPoints defining the region to crop from the source
    /// * `source` - The source ImageFrame to crop from
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(&'static str) if:
    ///   - The source and target frames have different channel counts
    ///   - The crop region would not fit in the source frame
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let mut target = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(50, 50));
    /// let source = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let corners = CornerPoints::new((10, 10), (50, 50)).unwrap();
    /// target.in_place_crop_and_nearest_neighbor_resize_to_self(&corners, &source).unwrap();
    /// ```
    pub fn in_place_crop_and_nearest_neighbor_resize_to_self(&mut self, source_cropping_points: &CornerPoints, source: &ImageFrame) -> Result<(), DataProcessingError> {
        let crop_resolution: (usize, usize) = source_cropping_points.enclosed_area();
        if &source.get_color_channel_count() != &self.get_color_channel_count() {
            return Err(DataProcessingError::IncompatibleInplace("The source and source do not have the same color channel count!".into()))
        }
        let source_full_resolution: (usize, usize) = source.get_xy_resolution();
        if !source_cropping_points.does_fit_in_frame_of_resolution(source_full_resolution){
            return Err(DataProcessingError::InvalidInputBounds("The upper left coordinate must be within the resolution range of the source image!".into()))
        }

        let resolution: (usize, usize) = self.get_xy_resolution();
        let resolution_f: (f32, f32) = (resolution.0 as f32, resolution.1 as f32);
        let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

        for ((x,y,c), color_val) in self.pixels.indexed_iter_mut() {
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / resolution_f.0) * crop_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / resolution_f.1) * crop_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = source.pixels[(nearest_neighbor_coordinate_x + source_cropping_points.lower_left().0, nearest_neighbor_coordinate_y + source_cropping_points.lower_left().1, c)]; // TODO ???
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(())
    }

    /// Calculates the thresholded difference between two frames and stores the result in this frame.
    ///
    /// This method computes the absolute difference between corresponding pixels in `previous_frame` and `next_frame`.
    /// If the difference exceeds the threshold, the pixel value is set to the difference value.
    /// Otherwise, the pixel value is set to 0.0.
    ///
    /// # Arguments
    ///
    /// * `previous_frame` - The first frame to compare
    /// * `next_frame` - The second frame to compare
    /// * `threshold` - The threshold value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(&'static str) if:
    ///   - The frames have different resolutions or channel counts
    ///   - The threshold is outside the valid range [0.0, 1.0]
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
    ///
    /// let mut diff_frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let prev_frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// let next_frame = ImageFrame::new(&ChannelFormat::RGB, &ColorSpace::Gamma, &(100, 100));
    /// diff_frame.in_place_calculate_difference_thresholded(&prev_frame, &next_frame, 0.1).unwrap();
    /// ```
    pub fn in_place_calculate_difference_thresholded(&mut self, previous_frame: &ImageFrame, next_frame: &ImageFrame, threshold: f32) -> Result<(), DataProcessingError> {
        if !ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(&previous_frame, next_frame) {
            return Err(DataProcessingError::IncompatibleInplace("The two given frames do not have equivalent resolutions or channel counts!".into()))
        }
        if !ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(self, next_frame) {
            return Err(DataProcessingError::IncompatibleInplace("This frame does not have equivalent resolutions or channel count to the given comparing frames!".into()))
        }
        if threshold > 1.0 || threshold < 0.0 {
            return Err(DataProcessingError::InvalidInputBounds("The threshold value must be between 0 and 1!".into()))
        }
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
    
    
    
     */

}