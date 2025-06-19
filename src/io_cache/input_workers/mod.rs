// Handles processing sensor data into neuron data

pub mod float_input_workers;

use crate::error::DataProcessingError;
use crate::brain_input::vision::image_frame;
use crate::brain_input::vision::image_frame::ImageFrame;
use crate::neuron_data::neuron_mappings::CorticalMappedXYZPNeuronData;

pub trait InputCacheWorker {
    fn get_as_cortical_mapped_xyzp_neuron_data() -> Result<CorticalMappedXYZPNeuronData, DataProcessingError>; // NOTE: Cortical area(s) is cached in the struct itself
    
    fn get_last_updated_timestamp() -> u128;
    
}

pub trait InputFloatCacheWorker {
    fn update_sensor_value(sensor_value: f32) -> Result<(), DataProcessingError>;
}

pub trait InputImageFrameWorker {
    fn update_sensor_value(sensor_value: ImageFrame) -> Result<(), DataProcessingError>;
}

