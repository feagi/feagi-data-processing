// Handles processing sensor data into neuron data

use crate::error::DataProcessingError;
use crate::data_types::ImageFrame;
use crate::data_types::neuron_data::CorticalMappedXYZPNeuronData;
use crate::data_types::neuron_data::NeuronTranslator;
use super::IOCacheWorker;

pub mod float_input_workers;
pub mod image_input_workers;

// NOTE: The reason we will have CorticalID and channel remain as a copied property for workers instead
// of an input parameter is because some workers generate multiple cortical areas worth of neurons
// and others only one, and trying to it that into these traits would be a pain in the ass
trait InputCacheWorker<T>: IOCacheWorker<T> {
    fn write_to_cortical_mapped_xyzp_neuron_data(&self, translator: &dyn NeuronTranslator<T>, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), DataProcessingError>;
    
    fn update_sensor_value(&mut self, sensor_value: T) -> Result<(), DataProcessingError>;
    
    fn get_last_stored_sensor_value(&self) -> Result<&T, DataProcessingError>;
}


