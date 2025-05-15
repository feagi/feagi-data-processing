use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::ImageFrame;
use std::fs::File;
use ndarray::Array3;
use ndarray_npy::NpzReader;
use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::ColorSpace;

#[test]
fn test_loading_data_frame() {
    let npz_file = File::open("tests/boxes_image_frame_raw.npz").expect("Failed to open NPZ file!");
    let mut npz = NpzReader::new(npz_file).expect("Failed to read NPZ file!");
    let source_array: Array3<f32> = npz.by_name("arr_0.npy").expect("Not able to read array!");
    assert_eq!(source_array.ndim(), 3); // If this fails, something is wrong loading the file at all
    
    let color_space: ColorSpace = ColorSpace::Linear; // Just Guessing
    let source_frame = ImageFrame::from_array(color_space, source_array);
    assert_eq!(source_frame.unwrap().get_xy_resolution(), (320, 240));
    
    

}

