// Handles processing sensor data into neuron data

pub mod float_input_workers;

use crate::error::DataProcessingError;
use crate::data_types::ImageFrame;
use crate::data_types::neuron_data::CorticalMappedXYZPNeuronData;

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

