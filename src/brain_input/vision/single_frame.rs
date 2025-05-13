use ndarray::{s, Array3, ArrayView3};
pub use crate::brain_input::vision::cropping_utils::CornerPoints;


/// Represents the color channel format of an image.
///
/// This enum defines the possible color channel configurations for an image:
/// - GrayScale: Single channel (grayscale, or red)
/// - RG: Two channels (red, green)
/// - RGB: Three channels (red, green, blue)
/// - RGBA: Four channels (red, green, blue, alpha)
#[derive(Clone)]
pub enum ChannelFormat {
    GrayScale, // R
    RG,
    RGB,
    RGBA,
}

/// A structure representing an image frame with pixel data and channel format information.
///
/// Various functions exist for processing images for use with FEAGI
/// Internally, uses a 3D ndarray of f32 values from 0-1
///
/// # Examples
///
/// ```
/// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat};
///
/// // Create a new RGB image frame
/// let resolution = (640, 480);
/// let frame = ImageFrame::new(&ChannelFormat::RGB, &resolution);
///
/// // Get the image resolution
/// assert_eq!(frame.get_xy_resolution(), resolution);
/// ```
pub struct ImageFrame {
    pixels: Array3<f32>,
    channel_format: ChannelFormat,
}

impl ImageFrame {
    /// Creates a new ImageFrame with the specified channel format and resolution.
    ///
    /// # Arguments
    ///
    /// * `channel_format` - The color channel format for the image
    /// * `xy_resolution` - The resolution of the image as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A new ImageFrame instance with all pixels initialized to zero.
    pub fn new(channel_format: &ChannelFormat, xy_resolution: &(usize, usize)) -> ImageFrame {
        ImageFrame {
            pixels: match channel_format {
                ChannelFormat::GrayScale => Array3::<f32>::zeros((xy_resolution.0, xy_resolution.1, 1)),
                ChannelFormat::RG => Array3::<f32>::zeros((xy_resolution.0, xy_resolution.1, 2)),
                ChannelFormat::RGB => Array3::<f32>::zeros((xy_resolution.0, xy_resolution.1, 3)),
                ChannelFormat::RGBA => Array3::<f32>::zeros((xy_resolution.0, xy_resolution.1, 4)),
            },
            channel_format: channel_format.clone(),
        }
    }

    /// Creates an ImageFrame from an existing ndarray.
    ///
    /// # Arguments
    ///
    /// * `input` - A 3D array of f32 values representing the image pixels
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(ImageFrame) if the input array has a valid number of color channels (1-4)
    /// - Err(&'static str) if the number of color channels is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array3;
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
    ///
    /// let array = Array3::<f32>::zeros((100, 100, 3)); // RGB image
    /// let frame = ImageFrame::from_array(array).unwrap();
    /// ```
    pub fn from_array(input: Array3<f32>) -> Result<ImageFrame, &'static str> {
        let number_color_channels: usize = input.shape()[2];
        Ok(ImageFrame {
            pixels: input,
            channel_format: match number_color_channels {
                1 => ChannelFormat::GrayScale,
                2 => ChannelFormat::RG,
                3 => ChannelFormat::RGB,
                4 => ChannelFormat::RGBA,
                _ => return Err("The number of color channels must be at least 1 and not exceed the 4!")
            }
        })
    }

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
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat, CornerPoints};
    ///
    /// let source = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// let corners = CornerPoints::new((10, 10), (50, 50)).unwrap();
    /// let cropped = ImageFrame::from_source_frame_crop(&source, &corners).unwrap();
    /// ```
    pub fn from_source_frame_crop(source_frame: &ImageFrame, corners_crop: &CornerPoints) -> Result<ImageFrame, &'static str> {
        let source_resolution = source_frame.get_xy_resolution();
        if !corners_crop.does_fit_in_frame_of_resolution(source_resolution) {
            return Err("The given crop would not fit in the given source!");
        }
        let channel_count: usize = source_frame.get_color_channel_count();
        let sliced_array_view: ArrayView3<f32> = source_frame.pixels.slice(s![corners_crop.lower_left.0 .. corners_crop.upper_right.0, corners_crop.lower_left.1 .. corners_crop.upper_right.1 , 0..channel_count]);
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
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat, CornerPoints};
    ///
    /// let source = ImageFrame::new(&ChannelFormat::RGB, &(300, 300));
    /// let corners = CornerPoints::new((10, 10), (50, 50)).unwrap();
    /// let resized = ImageFrame::from_source_frame_crop_and_resize(&source, &corners, &(200, 200)).unwrap();
    /// ```
    pub fn from_source_frame_crop_and_resize(source_frame: &ImageFrame, corners_crop: &CornerPoints, new_resolution: &(usize, usize)) -> Result<ImageFrame, &'static str> {
        let source_resolution = source_frame.get_xy_resolution();
        if !corners_crop.does_fit_in_frame_of_resolution(source_resolution) {
            return Err("The given crop would not fit in the given source!");
        }
        let channel_count: usize = source_frame.get_color_channel_count();

        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32);
        let crop_resolution_f: (f32, f32) = (new_resolution.0 as f32, new_resolution.1 as f32);
        let mut writing_array: Array3<f32> = Array3::<f32>::zeros((new_resolution.0, new_resolution.1, channel_count));

        for ((x,y,c), color_val) in writing_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / source_resolution_f.0) * crop_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / source_resolution_f.1) * crop_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = source_frame.pixels[(nearest_neighbor_coordinate_x + corners_crop.lower_left.0, nearest_neighbor_coordinate_y + corners_crop.lower_right().1, c)];
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
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat, CornerPoints};
    ///
    /// let mut target = ImageFrame::new(&ChannelFormat::RGB, &(50, 50));
    /// let source = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// let corners = CornerPoints::new((10, 10), (50, 50)).unwrap();
    /// target.in_place_crop_and_nearest_neighbor_resize_to_self(&corners, &source).unwrap();
    /// ```
    pub fn in_place_crop_and_nearest_neighbor_resize_to_self(&mut self, source_cropping_points: &CornerPoints, source: &ImageFrame) -> Result<(), &'static str> {
        let crop_resolution: (usize, usize) = source_cropping_points.enclosed_area();
        if &source.get_color_channel_count() != &self.get_color_channel_count() {
            return Err("The source and source do not have the same color channel count!");
        }
        let source_full_resolution: (usize, usize) = source.get_xy_resolution();
        if !source_cropping_points.does_fit_in_frame_of_resolution(source_full_resolution){
            return Err("The upper left coordinate must be within the resolution range of the source image!");
        }
        
        let resolution: (usize, usize) = self.get_xy_resolution();
        let resolution_f: (f32, f32) = (resolution.0 as f32, resolution.1 as f32);
        let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

        for ((x,y,c), color_val) in self.pixels.indexed_iter_mut() {
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / resolution_f.0) * crop_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / resolution_f.1) * crop_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = source.pixels[(nearest_neighbor_coordinate_x + source_cropping_points.lower_left.0, nearest_neighbor_coordinate_y + source_cropping_points.lower_left.1, c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(())
    }

    /// Returns true if 2 ImageFrames have the same channel count and resolution
    pub fn are_two_image_frames_compatible(a: &ImageFrame, b: &ImageFrame) -> bool {
        a.get_color_channel_count() == b.get_color_channel_count() && a.get_xy_resolution() == b.get_xy_resolution()
    }

    /// Returns a reference to the channel format of this image.
    pub fn get_channel_format(&self) -> &ChannelFormat {
        &self.channel_format
    }

    /// Returns the number of color channels of this ImageFrame
    pub fn get_color_channel_count(&self) -> usize {
        match self.channel_format {
            ChannelFormat::GrayScale => 1,
            ChannelFormat::RG => 2,
            ChannelFormat::RGB => 3,
            ChannelFormat::RGBA => 4,
        }
    }

    /// Returns a view of the pixel data.
    ///
    /// This provides read-only access to the underlying pixel array.
    pub fn get_view_pixels(&self) -> ArrayView3<f32> {
        self.pixels.view()
    }

    /// Returns the resolution of the image as a tuple of (width, height).
    pub fn get_xy_resolution(&self) -> (usize, usize) {
        let shape: &[usize] =  self.pixels.shape();
        (shape[0], shape[1])
    }

    
    /// Adjusts the brightness of the image by multiplying each pixel value by a positive factor.
    ///
    /// # Arguments
    ///
    /// * `brightness_factor` - The factor to multiply each pixel value by
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(&'static str) if the brightness factor is negative
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat};
    ///
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// frame.change_brightness_multiplicative(1.5).unwrap(); // Increase brightness by 50%
    /// ```
    pub fn change_brightness_multiplicative(&mut self, brightness_factor: f32) -> Result<(), &'static str> {
        if brightness_factor < 0.0 {
            return Err("The brightness factor cannot be negative!");
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
    /// that preserves the middle gray value (128) while stretching or compressing the
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
    /// - Err(&'static str) if the contrast factor is outside the valid range of -1 to 1
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat};
    ///
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// frame.change_contrast(0.5).unwrap();  // Increase contrast
    /// frame.change_contrast(-0.3).unwrap(); // Decrease contrast
    /// frame.change_contrast(0.0).unwrap(); // Do nothing (0 is the starting point)
    /// ```
    pub fn change_contrast(&mut self, contrast_factor: f32) -> Result<(), &'static str> {
        if contrast_factor < -1.0 || contrast_factor > 1.0 {
            return Err("The contrast factor must be between -1.0 and 1.0!");
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

    
    /// Resizes the image using nearest neighbor. Low quality, but fast.
    ///
    /// Color channel information is preserved.
    ///
    /// # Arguments
    ///
    /// * `target_resolution` - The desired resolution as a tuple of (width, height)
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// - Ok(()) if the operation was successful
    /// - Err(&'static str) if the target resolution is invalid (zero or negative)
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat};
    ///
    /// let mut frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// frame.resize_nearest_neighbor(&(200, 200)).unwrap(); // Double the resolution
    /// ```
    pub fn resize_nearest_neighbor(&mut self, target_resolution: &(usize, usize)) -> Result<(), &'static str> {
        if target_resolution.0 <= 0 || target_resolution.1 <= 0 {
            return Err("The resolution factor cannot be zero or negative!");
        }
        let source_resolution: (usize, usize) = self.get_xy_resolution();
        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32);
        let number_color_channels: usize = self.get_color_channel_count();

        let mut sized_array: Array3<f32> = Array3::zeros((target_resolution.0, target_resolution.1, number_color_channels));
        let target_resolution_f: (f32, f32) = (target_resolution.0 as f32, target_resolution.1 as f32);
        for ((x,y,c), color_val) in sized_array.indexed_iter_mut() {
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / target_resolution_f.0) * source_resolution_f.0).floor() as usize;
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / target_resolution_f.1) * source_resolution_f.1).floor() as usize;
            let nearest_neighbor_channel_value: f32 = self.pixels[(nearest_neighbor_coordinate_x, nearest_neighbor_coordinate_y, c)];
            *color_val = nearest_neighbor_channel_value;
        };
        self.pixels = sized_array;
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
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat};
    ///
    /// let mut diff_frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// let prev_frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// let next_frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// diff_frame.in_place_calculate_difference_thresholded(&prev_frame, &next_frame, 0.1).unwrap();
    /// ```
    pub fn in_place_calculate_difference_thresholded(&mut self, previous_frame: &ImageFrame, next_frame: &ImageFrame, threshold: f32) -> Result<(), &'static str> {
        if !ImageFrame::are_two_image_frames_compatible(&previous_frame, next_frame) {
            return Err("The two given frames do not have equivalent resolutions or channel counts!");
        }
        if !ImageFrame::are_two_image_frames_compatible(self, next_frame) {
            return Err("This frame does not have equivalent resolutions or channel count to the given comparing frames!");
        }
        if threshold > 1.0 || threshold < 0.0 {
            return Err("The threshold value must be between 0 and 1!");
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

    /// Calculates the number of bytes needed to store the XYZP (coordinates and potential) data.
    ///
    /// Each voxel (pixel) requires 16 bytes of storage:
    /// - 4 bytes for X coordinate (u32)
    /// - 4 bytes for Y coordinate (u32)
    /// - 4 bytes for Z coordinate (u32)
    /// - 4 bytes for potential value (f32)
    ///
    /// # Returns
    ///
    /// The total number of bytes needed to store all voxel data
    pub fn get_number_of_bytes_needed_to_hold_xyzp_uncompressed(& self) -> usize {
        const NUMBER_BYTES_PER_VOXEL: usize = 16;
        let dimensions = self.pixels.shape(); // we know its 3 elements
        dimensions[0] * dimensions[1] * dimensions[2] * NUMBER_BYTES_PER_VOXEL
    }

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
    /// A Vec<u8> containing the serialized XYZP data
    pub fn to_bytes(& self) -> Vec<u8> {
        let required_number_elements = self.get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
        let mut output: Vec<u8> = Vec::with_capacity(required_number_elements);
        
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
    /// - Err(&'static str) if the provided buffer is too small
    ///
    /// # Examples
    ///
    /// ```
    /// use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame, ChannelFormat};
    ///
    /// let frame = ImageFrame::new(&ChannelFormat::RGB, &(100, 100));
    /// let mut buffer = vec![0u8; frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed()];
    /// frame.to_bytes_in_place(&mut buffer).unwrap();
    /// ```
    pub fn to_bytes_in_place(& self, bytes_writing_to: &mut [u8]) -> Result<(), &'static str> {
        let required_capacity: usize = self.get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
        if bytes_writing_to.len() < required_capacity {
            return Err("Given buffer is too small!");
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



}