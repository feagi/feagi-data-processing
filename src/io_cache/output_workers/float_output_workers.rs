use std::time::Instant;
use crate::data_types::{RangedNormalizedF32, LinearBoundedF32};
use crate::error::DataProcessingError;
use crate::genome_definitions::identifiers::CorticalID;
use crate::io_cache::{ChannelIndex, IOCacheWorker};
use crate::io_cache::callback_manager::CallBackManager;
use super::{OutputCacheWorker};

pub trait OutputFloatCacheWorker: OutputCacheWorker<LinearBoundedF32> {

}

//region Float Direct
pub struct FloatDirectWorker {
    last_data_update_time: Instant,
    cortical_id_write_target: CorticalID,
    callback_manager: CallBackManager<LinearBoundedF32>,
    channel: ChannelIndex,
    most_recent_float: LinearBoundedF32,
}

impl IOCacheWorker<LinearBoundedF32> for FloatDirectWorker {
    fn get_last_data_update_time(&self) -> Instant {
        self.last_data_update_time
    }
}

impl OutputCacheWorker<LinearBoundedF32> for FloatDirectWorker {
    fn get_last_stored_motor_value(&self) -> LinearBoundedF32 {
        self.most_recent_float
    }

    fn feagi_sent_value(&mut self, new_value: LinearBoundedF32) {
        self.last_data_update_time = Instant::now();
        self.most_recent_float = new_value; // Don't do callbacks here, that will be handled higher level
    }

    fn callback_manager(&mut self) -> &mut CallBackManager<LinearBoundedF32> {
        &mut self.callback_manager
    }
}

impl OutputFloatCacheWorker for FloatDirectWorker {

}

impl FloatDirectWorker {
    pub fn new(cortical_id_write_target: CorticalID, channel: ChannelIndex, starting_value: f32, upper_bound: f32, lower_bound: f32) -> Result<FloatDirectWorker, DataProcessingError> {
        let start_val: LinearBoundedF32 = LinearBoundedF32::new(starting_value, upper_bound, lower_bound)?;
        let current_time = Instant::now();
        let callback_manager: CallBackManager<LinearBoundedF32> = CallBackManager::new();
        Ok(FloatDirectWorker{
            last_data_update_time: current_time,
            cortical_id_write_target: cortical_id_write_target,
            callback_manager,
            channel,
            most_recent_float: start_val,
        })
    }
}
