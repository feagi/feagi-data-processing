use crate::error::DataProcessingError;
use crate::data_types::neuron_data::CorticalMappedXYZPNeuronData;
use super::IOCacheWorker;

pub type CallbackSubscriberID = usize;

trait OutputCacheWorker<T>: IOCacheWorker<T> {

    fn get_stored_value(&self) -> T;
    fn subscribe_to_callback<F>(&mut self, callback: F) -> Result<(), DataProcessingError>
    where
        F: Fn(T) + Send  + Sync + 'static;
    
    fn unsubscribe_from_callback(&mut self, callback: CallbackSubscriberID) -> Result<(), DataProcessingError>;

    fn unsubscribe_from_all_callbacks(&mut self) -> Result<(), DataProcessingError>;
}

trait OutputFloatCacheWorker: OutputCacheWorker<f32> {
    
}
