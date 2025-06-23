use std::time::Instant;
use crate::data_types::neuron_data::CorticalMappedXYZPNeuronData;
use crate::error::DataProcessingError;
use crate::io_cache::IOCacheWorker;
use crate::data_types::neuron_data::{XYZPFloatTranslator, XYZPFloatTranslatorType};
use super::{InputCacheWorker, InputFloatCacheWorker};

//region Float Direct
pub struct FloatDirectWorker<'translator> {
    last_data_update_time: Instant,
    last_float: f32,
    translator_ref: &'translator XYZPFloatTranslator
}

impl IOCacheWorker<f32> for FloatDirectWorker<'_> {
    fn get_last_data_update_time(&self) -> Instant {
        self.last_data_update_time
    }
}

impl InputCacheWorker<f32> for FloatDirectWorker<'_> {
    fn get_as_cortical_mapped_xyzp_neuron_data(&self) -> Result<CorticalMappedXYZPNeuronData, DataProcessingError> {
        Err(DataProcessingError::NotImplemented) // TODO
    }

    fn update_sensor_value(&mut self, sensor_value: f32) -> Result<(), DataProcessingError> {
        self.last_float = sensor_value;
        self.last_data_update_time = Instant::now();
        Ok(())
    }
}

impl InputFloatCacheWorker for FloatDirectWorker<'_> {
    
}

impl FloatDirectWorker<'_> {
    pub fn new(float_ipu_type: XYZPFloatTranslatorType, resolution_depth: usize) -> Result<FloatDirectWorker, DataProcessingError> {
        let translator = XYZPFloatTranslator::new(float_ipu_type, number_channels, resolution_depth)?;
        Ok(FloatDirectWorker{
            last_data_update_time: Instant::now(),
            last_float: 0f32,
            translator
        })
    }
}
//endregion

