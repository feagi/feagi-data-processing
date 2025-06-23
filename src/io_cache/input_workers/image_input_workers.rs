use std::time::Instant;
use crate::data_types::neuron_data::CorticalMappedXYZPNeuronData;
use crate::error::DataProcessingError;
use crate::io_cache::IOCacheWorker;
use crate::data_types::{ImageFrame, SegmentedVisionFrame};
use crate::io_cache::input_workers::{InputCacheWorker, InputImageFrameWorker};

//region Image Direct
pub struct ImageDirectWorker {
    last_data_update_time: Instant,
    last_image: ImageFrame
}

impl InputCacheWorker for ImageDirectWorker {
    fn get_as_cortical_mapped_xyzp_neuron_data(&self) -> Result<CorticalMappedXYZPNeuronData, DataProcessingError> {
        todo!()
    }
}

impl IOCacheWorker for ImageDirectWorker {
    fn get_last_data_update_time(&self) -> Instant {
        todo!()
    }

    fn get_channel_enable_state(&self) -> Vec<bool> {
        todo!()
    }

    fn get_enabled_channels(&self) -> Vec<usize> {
        todo!()
    }

    fn get_disabled_channels(&self) -> Vec<usize> {
        todo!()
    }

    fn is_channel_enabled(&self, channel: usize) -> bool {
        todo!()
    }
}

impl InputImageFrameWorker for ImageDirectWorker {
    fn update_sensor_value(&mut self, sensor_value: ImageFrame) -> Result<(), DataProcessingError> {
        todo!()
    }
}