use std::fs::File;
use std::time::Instant;
use ndarray::Array3;
use ndarray_npy::NpzReader;
use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalIOChannelIndex, SensorCorticalType, SingleChannelDimensions};
use feagi_core_data_structures_and_processing::io_data::image_descriptors::{ColorChannelLayout, ColorSpace, MemoryOrderLayout};
use feagi_core_data_structures_and_processing::io_data::ImageFrame;
use feagi_core_data_structures_and_processing::io_processing::processors::{IdentityImageFrameProcessor};
use feagi_core_data_structures_and_processing::io_processing::{SensorCache, StreamCacheProcessor};
use feagi_core_data_structures_and_processing::io_processing::byte_structures::FeagiByteStructure;
use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;

/*
#[test]
fn test_simple_serialization() {
    let res = (4, 5);

    let array = Array3::ones((res.0, res.1, 3));
    let mut frame = ImageFrame::from_array(
        array,
        &ColorSpace::Linear,
        &MemoryOrderLayout::HeightsWidthsChannels
    ).unwrap();

    frame.change_brightness(10.0);
    assert_eq!(frame.get_internal_data()[(0,0,0)], 10.0);


    let cortical_type = SensorCorticalType::ImageCameraCenter;
    let cortical_group = CorticalGroupingIndex::from(4);
    let number_channels = 1;
    let device_channel_dimensions = SingleChannelDimensions::new(res.0 as u32, res.1 as  u32, 3).unwrap();
    let device_channel_index = CorticalIOChannelIndex::from(0);

    let mut input_processors: Vec<Box<dyn StreamCacheProcessor + Sync + Send>> = vec![];
    input_processors.push(Box::new(IdentityImageFrameProcessor::new(frame.clone()).unwrap()));

    let allow_old_data: bool = false;

    let mut sensor_cache = SensorCache::new();
    sensor_cache.register_single_cortical_area(cortical_type, cortical_group, number_channels, device_channel_dimensions).unwrap();
    sensor_cache.register_single_channel(cortical_type, cortical_group, device_channel_index, input_processors, allow_old_data).unwrap();

    let mut neurons: CorticalMappedXYZPNeuronData = CorticalMappedXYZPNeuronData::new();
    sensor_cache.encode_to_neurons(Instant::now(), &mut neurons).unwrap();
    let feagi_bytes = FeagiByteStructure::create_from_compatible(Box::new(neurons)).unwrap();
    let bytes = feagi_bytes.copy_out_as_byte_vector();
    assert_eq!(bytes.len(), 978);
}

 */


#[test]
fn test_loading_data_frame() {
    let npz_file = File::open("tests/boxes_image_frame_raw.npz").expect("Failed to open NPZ file!");
    let mut npz = NpzReader::new(npz_file).expect("Failed to read NPZ file!");
    let source_array: Array3<f32> = npz.by_name("arr_0.npy").expect("Not able to read array!");
    assert_eq!(source_array.ndim(), 3); // If this fails, something is wrong loading the file at all
    
    //let color_space: ColorSpace = ColorSpace::Linear; // Just Guessing
    //let memory_order: MemoryOrderLayout = MemoryOrderLayout::HeightsWidthsChannels; // the standard row major
    //let _source_frame = ImageFrame::from_array(source_array, color_space, memory_order);
    //assert_eq!(source_frame.unwrap().get_xy_resolution(), (320, 240));
    
    

}

