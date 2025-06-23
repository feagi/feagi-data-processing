// Handles processing sensor data into neuron data

use crate::error::DataProcessingError;
use crate::data_types::ImageFrame;
use crate::data_types::neuron_data::CorticalMappedXYZPNeuronData;
use super::IOCacheWorker;

pub mod float_input_workers;
pub mod image_input_workers;

trait InputCacheWorker<T>: IOCacheWorker<T> {
    fn get_as_cortical_mapped_xyzp_neuron_data(&self) -> Result<CorticalMappedXYZPNeuronData, DataProcessingError>;
    
    fn update_sensor_value(&mut self, sensor_value: T) -> Result<(), DataProcessingError>;
    
}

pub trait InputFloatCacheWorker: InputCacheWorker<f32> {
    
}

pub trait InputImageFrameWorker: InputCacheWorker<ImageFrame> {

}

