use std::time::Instant;
use crate::data_types::{RangedNormalizedF32, LinearBoundedF32};
use crate::error::DataProcessingError;
use crate::genome_definitions::identifiers::CorticalID;
use crate::io_cache::{ChannelIndex, IOCacheWorker};
use super::{CallbackSubscriberID, OutputCacheWorker};

pub trait OutputFloatCacheWorker: OutputCacheWorker<LinearBoundedF32> {
    
}

//region Float Direct
pub struct FloatDirectWorker {
    last_data_update_time: Instant,
    cortical_id_write_target: CorticalID,
    channel: ChannelIndex,
    last_float: LinearBoundedF32,
}

impl IOCacheWorker<LinearBoundedF32> for FloatDirectWorker {
    fn get_last_data_update_time(&self) -> Instant {
        self.last_data_update_time
    }
}

impl OutputCacheWorker<LinearBoundedF32> for FloatDirectWorker {
    fn get_last_stored_motor_value(&self) -> LinearBoundedF32 {
        self.last_float
    }

    fn feagi_sent_value(&mut self, new_value: LinearBoundedF32) {
        self.last_float = new_value; // Don't do callbacks here, that will be handled higher level
    }

    fn call_all_callbacks(&self) {
        todo!()
    }

    fn subscribe_to_callback<F>(&mut self, callback: F) -> Result<(), DataProcessingError>
    where
        F: Fn(LinearBoundedF32) + Send + Sync + 'static
    {
        todo!()
    }

    fn unsubscribe_from_callback(&mut self, callback: CallbackSubscriberID) -> Result<(), DataProcessingError> {
        todo!()
    }

    fn unsubscribe_from_all_callbacks(&mut self) -> Result<(), DataProcessingError> {
        todo!()
    }
}

impl OutputFloatCacheWorker for FloatDirectWorker {
    
}

impl FloatDirectWorker {
    
}
