// Module for image data structures for FEAGI. Essentially pixel data stored uncompressed in ndarrays

pub mod peripheral_segmentation;
pub mod cropping_utils;
pub mod single_frame;





#[cfg(test)]
pub mod tests {
    use super::*;
    use single_frame::{ImageFrame, ChannelFormat};
    use ndarray::{Array3};

    #[test]
    fn test_get_view_pixels() {
        let resolution = (1920, 1080);

        // R
        let frame = ImageFrame::new(&ChannelFormat::GrayScale, &resolution);
        let view = frame.get_view_pixels();
        assert_eq!(view.shape(), [resolution.0, resolution.1, 1]);

        // RG
        let frame = ImageFrame::new(&ChannelFormat::RG, &resolution);
        let view = frame.get_view_pixels();
        assert_eq!(view.shape(), [resolution.0, resolution.1, 2]);

        // RGB
        let frame = ImageFrame::new(&ChannelFormat::RGB, &resolution);
        let view = frame.get_view_pixels();
        assert_eq!(view.shape(), [resolution.0, resolution.1, 3]);

        // RGBA
        let frame = ImageFrame::new(&ChannelFormat::RGBA, &resolution);
        let view = frame.get_view_pixels();
        assert_eq!(view.shape(), [resolution.0, resolution.1, 4]);
    }

    #[test]
    fn test_change_brightness_multiplicative() {
        // Create a test image with known values
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G
        test_array[[0, 0, 2]] = 0.8;  // B
        
        // Create ImageFrame from array
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        
        // Test brightness increase
        frame.change_brightness_multiplicative(1.5).unwrap();
        let view = frame.get_view_pixels();
        assert_eq!(view[[0, 0, 0]], 0.3);  // 0.2 * 1.5 = 0.3
        assert_eq!(view[[0, 0, 1]], 0.75); // 0.5 * 1.5 = 0.75
        assert_eq!(view[[0, 0, 2]], 1.0);  // 0.8 * 1.5 = 1.2, but clamped to 1.0
        
        // Test brightness decrease
        frame.change_brightness_multiplicative(0.5).unwrap();
        let view = frame.get_view_pixels();
        assert_eq!(view[[0, 0, 0]], 0.15); // 0.3 * 0.5 = 0.15
        assert_eq!(view[[0, 0, 1]], 0.375); // 0.75 * 0.5 = 0.375
        assert_eq!(view[[0, 0, 2]], 0.5);  // 1.0 * 0.5 = 0.5
        
        // Test zero brightness
        frame.change_brightness_multiplicative(0.0).unwrap();
        let view = frame.get_view_pixels();
        assert_eq!(view[[0, 0, 0]], 0.0);
        assert_eq!(view[[0, 0, 1]], 0.0);
        assert_eq!(view[[0, 0, 2]], 0.0);
        
        // Test negative brightness (should return error)
        let result = frame.change_brightness_multiplicative(-1.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The brightness factor cannot be negative!");
    }

    #[test]
    fn test_change_contrast() {
        const EPSILON: f32 = 0.0001;
        
        // Create a test image with known values
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G (middle value)
        test_array[[0, 0, 2]] = 0.8;  // B
        
        // Create ImageFrame from array
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        
        // Test maximum contrast increase
        frame.change_contrast(1.0).unwrap();
        let view = frame.get_view_pixels();
        assert!(view[[0, 0, 0]].abs() < EPSILON);   // Dark values become darker
        assert!((view[[0, 0, 1]] - 0.5).abs() < EPSILON);   // Middle value stays the same
        assert!((view[[0, 0, 2]] - 1.0).abs() < EPSILON);   // Bright values become brighter
        
        // Test maximum contrast decrease
        frame.change_contrast(-1.0).unwrap();
        let view = frame.get_view_pixels();
        assert!((view[[0, 0, 0]] - 0.5).abs() < EPSILON);   // All values become middle gray
        assert!((view[[0, 0, 1]] - 0.5).abs() < EPSILON);
        assert!((view[[0, 0, 2]] - 0.5).abs() < EPSILON);
        
        // Test moderate contrast increase
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G (middle value)
        test_array[[0, 0, 2]] = 0.8;  // B
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        frame.change_contrast(0.5).unwrap();
        let view = frame.get_view_pixels();
        assert!((view[[0, 0, 0]]) < 0.2);  // Dark values become darker
        assert!((view[[0, 0, 1]] - 0.5).abs() < EPSILON); // Middle value stays the same
        assert!((view[[0, 0, 2]]) > 0.8);  // Bright values become brighter
        
        // Test moderate contrast decrease
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G (middle value)
        test_array[[0, 0, 2]] = 0.8;  // B
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        frame.change_contrast(-0.5).unwrap();
        let view = frame.get_view_pixels();
        assert!(view[[0, 0, 0]] > 0.2 && view[[0, 0, 0]] < 0.5);  // Dark values become lighter
        assert_eq!(view[[0, 0, 1]], 0.5); // Middle value stays the same
        assert!(view[[0, 0, 2]] < 0.8 && view[[0, 0, 2]] > 0.5); // Bright values become darker
        
        // Test zero contrast (should have no effect)
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G (middle value)
        test_array[[0, 0, 2]] = 0.8;  // B
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        frame.change_contrast(0.0).unwrap();
        let view = frame.get_view_pixels();
        assert!((view[[0, 0, 0]] - 0.2).abs() < EPSILON);
        assert!((view[[0, 0, 1]] - 0.5).abs() < EPSILON);
        assert!((view[[0, 0, 2]] - 0.8).abs() < EPSILON);
        
        // Test invalid contrast values
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G (middle value)
        test_array[[0, 0, 2]] = 0.8;  // B
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        let result = frame.change_contrast(1.1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The contrast factor must be between -1.0 and 1.0!");
        
        let result = frame.change_contrast(-1.1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The contrast factor must be between -1.0 and 1.0!");
    }

    #[test]
    fn test_resize_nearest_neighbor() {
        // Create a 2x2 RGB image with known values
        let mut test_array = Array3::<f32>::zeros((2, 2, 3));
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G
        test_array[[0, 0, 2]] = 0.8;  // B
        
        // Create ImageFrame from array
        let mut frame = ImageFrame::from_array(test_array).unwrap();
        
        // Resize to 4x4
        frame.resize_nearest_neighbor(&(4, 4)).unwrap();
        
        // Check the new dimensions
        assert_eq!(frame.get_xy_resolution(), (4, 4));
        
        // Check that the values were properly duplicated
        let view = frame.get_view_pixels();
        assert_eq!(view[[0, 0, 0]], 0.2);  // Original pixel
        assert_eq!(view[[1, 1, 0]], 0.2);  // Duplicated pixel
        assert_eq!(view[[2, 2, 0]], 0.0);  // Duplicated pixel
        assert_eq!(view[[3, 3, 0]], 0.0);  // Duplicated pixel
        
        // Test invalid resolution
        let result = frame.resize_nearest_neighbor(&(0, 4));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The resolution factor cannot be zero or negative!");
    }

    #[test]
    fn test_in_place_calculate_difference_thresholded() {
        const EPSILON: f32 = 0.0001;
        
        // Create three compatible frames with test values
        let mut prev_frame_pixels: Array3<f32> = Array3::<f32>::zeros((2, 2, 3));
        prev_frame_pixels[[0, 0, 0]] = 0.2;  // R
        prev_frame_pixels[[0, 0, 1]] = 0.5;  // G
        prev_frame_pixels[[0, 0, 2]] = 0.8;  // B

        let mut next_frame_pixels: Array3<f32> = Array3::<f32>::zeros((2, 2, 3));
        next_frame_pixels[[0, 0, 0]] = 0.4;  // R (diff: 0.2)
        next_frame_pixels[[0, 0, 1]] = 0.5;  // G (diff: 0.0)
        next_frame_pixels[[0, 0, 2]] = 0.6;  // B (diff: 0.2)


        let mut diff_frame = ImageFrame::new(&ChannelFormat::RGB, &(2, 2));
        let prev_frame = ImageFrame::from_array(prev_frame_pixels).unwrap();
        let next_frame = ImageFrame::from_array(next_frame_pixels).unwrap();

        
        // Test with threshold of 0.15
        diff_frame.in_place_calculate_difference_thresholded(&prev_frame, &next_frame, 0.15).unwrap();
        let view = diff_frame.get_view_pixels();
        
        // Check results
        assert!((view[[0, 0, 0]] - 0.2).abs() < EPSILON);  // Above threshold
        assert!((view[[0, 0, 1]] - 0.0).abs() < EPSILON);  // Below threshold
        assert!((view[[0, 0, 2]] - 0.2).abs() < EPSILON);  // Above threshold
        
        // Test with incompatible frames
        let incompatible_frame = ImageFrame::new(&ChannelFormat::RGB, &(3, 3));
        let result = diff_frame.in_place_calculate_difference_thresholded(&prev_frame, &incompatible_frame, 0.15);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The two given frames do not have equivalent resolutions or channel counts!");
        
        // Test with invalid threshold
        let result = diff_frame.in_place_calculate_difference_thresholded(&prev_frame, &next_frame, 1.1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The threshold value must be between 0 and 1!");
    }

    #[test]
    fn test_get_number_of_bytes_needed_to_hold_xyzp_uncompressed() {
        // Test with different resolutions and channel formats
        let resolutions = [(2, 2), (10, 10), (100, 100)];
        let channel_formats = [
            ChannelFormat::GrayScale,
            ChannelFormat::RG,
            ChannelFormat::RGB,
            ChannelFormat::RGBA,
        ];

        for resolution in resolutions {
            for format in &channel_formats {
                let frame = ImageFrame::new(format, &resolution);
                let expected_bytes = resolution.0 * resolution.1 * frame.get_color_channel_count() * 16; // 16 bytes per voxel
                assert_eq!(frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed(), expected_bytes);
            }
        }
    }

    #[test]
    fn test_to_bytes() {
        // Create a small test image with known values
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G
        test_array[[0, 0, 2]] = 0.8;  // B
        
        let frame = ImageFrame::from_array(test_array).unwrap();
        let bytes = frame.to_bytes();
        
        // Check total length
        assert_eq!(bytes.len(), frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed());
        
        // Check X coordinates (first quarter of bytes)
        let x_section = &bytes[0..bytes.len()/4];
        assert_eq!(u32::from_le_bytes(x_section[0..4].try_into().unwrap()), 0); // First X coord
        assert_eq!(u32::from_le_bytes(x_section[4..8].try_into().unwrap()), 1); // Second X coord
        
        // Check Y coordinates (second quarter of bytes)
        let y_section = &bytes[bytes.len()/4..bytes.len()/2];
        assert_eq!(u32::from_le_bytes(y_section[0..4].try_into().unwrap()), 0); // First Y coord
        assert_eq!(u32::from_le_bytes(y_section[4..8].try_into().unwrap()), 0); // Second Y coord
        
        // Check Z coordinates (third quarter of bytes)
        let z_section = &bytes[bytes.len()/2..3*bytes.len()/4];
        assert_eq!(u32::from_le_bytes(z_section[0..4].try_into().unwrap()), 0); // First Z coord (R channel)
        assert_eq!(u32::from_le_bytes(z_section[4..8].try_into().unwrap()), 1); // Second Z coord (G channel)
        
        // Check potential values (last quarter of bytes)
        let p_section = &bytes[3*bytes.len()/4..];
        assert!((f32::from_le_bytes(p_section[0..4].try_into().unwrap()) - 0.2).abs() < 0.0001); // First potential
        assert!((f32::from_le_bytes(p_section[4..8].try_into().unwrap()) - 0.5).abs() < 0.0001); // Second potential
    }

    #[test]
    fn test_to_bytes_in_place() {
        // Create a small test image with known values
        let mut test_array = Array3::<f32>::zeros((2, 2, 3)); // 2x2 RGB image
        test_array[[0, 0, 0]] = 0.2;  // R
        test_array[[0, 0, 1]] = 0.5;  // G
        test_array[[0, 0, 2]] = 0.8;  // B
        
        let frame = ImageFrame::from_array(test_array).unwrap();
        
        // Test with buffer of correct size
        let mut buffer = vec![0u8; frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed()];
        assert!(frame.to_bytes_in_place(&mut buffer).is_ok());
        
        // Verify the contents match to_bytes()
        let expected_bytes = frame.to_bytes();
        assert_eq!(buffer, expected_bytes);
        
        // Test with buffer too small
        let mut small_buffer = vec![0u8; frame.get_number_of_bytes_needed_to_hold_xyzp_uncompressed() - 1];
        let result = frame.to_bytes_in_place(&mut small_buffer);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Given buffer is too small!");
    }
}
