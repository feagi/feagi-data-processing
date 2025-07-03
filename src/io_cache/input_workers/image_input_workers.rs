use std::time::Instant;
use crate::data_types::neuron_data::{CorticalMappedXYZPNeuronData, NeuronTranslator};
use crate::error::DataProcessingError;
use crate::io_cache::{ChannelIndex, IOCacheWorker};
use crate::data_types::{ImageFrame, SegmentedVisionFrame};
use crate::genome_definitions::identifiers::CorticalID;
use crate::io_cache::input_workers::{InputCacheWorker};

pub trait InputImageFrameWorker: InputCacheWorker<ImageFrame> {
    // TODO other update methods that can be done in place
}

//region Image Direct
pub struct ImageDirectWorker {
    last_data_update_time: Instant,
    cortical_id_write_target: CorticalID, // yes, lets keep a copy here, this is too small to worry about borrowing shenanigans
    channel: ChannelIndex,
    last_image: ImageFrame,
}

impl IOCacheWorker<ImageFrame> for ImageDirectWorker {
    fn get_last_data_update_time(&self) -> Instant {
        self.last_data_update_time
    }
}

impl InputCacheWorker<ImageFrame> for ImageDirectWorker {
    fn write_to_cortical_mapped_xyzp_neuron_data(&self, translator: &dyn NeuronTranslator<ImageFrame>, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        todo!()
    }

    fn update_sensor_value(&mut self, sensor_value: ImageFrame) -> Result<(), FeagiDataProcessingError> {
        // NOTE: This method is rather slow but is here for completeness, it may be better to use other methods which can do in place operations
        self.last_image = sensor_value;
        self.last_data_update_time = Instant::now();
        Ok(())
    }

    fn get_last_stored_sensor_value(&self) -> Result<&ImageFrame, FeagiDataProcessingError> {
        Ok(&self.last_image)
    }
}

impl InputImageFrameWorker for ImageDirectWorker {

}