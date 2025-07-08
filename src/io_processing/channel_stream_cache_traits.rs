use std::time::Instant;
use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::CorticalIOChannelIndex;
use crate::io_processing::StreamCacheProcessor;
use crate::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZPEncoder};

pub trait SensoryChannelStreamCache<T: std::fmt::Display + Clone> {
    
    fn new(stream_cache_processor: Box<dyn StreamCacheProcessor<T>>,
           neuron_xyzp_encoder: Box<dyn NeuronXYZPEncoder<T>>,
           channel: CorticalIOChannelIndex,
    ) -> Result<Box<dyn SensoryChannelStreamCache<T>>, FeagiDataProcessingError> where Self: Sized;
    
    fn update_sensor_value(&mut self, value: T) -> Result<(), FeagiDataProcessingError>;

    fn get_most_recent_sensor_value(&self) -> &T;
    
    fn get_last_updated_time(&self) -> Instant;

    fn encode_to_neurons(&self, neuron_xyzp_arrays: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError>;
    
    fn is_more_recent_than_given(&self, time: Instant) -> bool {
        time < self.get_last_updated_time()
    }
}