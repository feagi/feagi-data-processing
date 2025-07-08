use std::time::{Instant};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_processing::{CallBackManager, StreamCacheProcessor};
use crate::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZPDecoder, NeuronXYZPEncoder};






pub struct SensoryChannelStreamCache<T: std::fmt::Display + Clone> {
    stream_cache_processor: Box<dyn StreamCacheProcessor<T>>,
    neuron_xyzp_encoder:  Box< dyn NeuronXYZPEncoder<T>>,
    channel: CorticalIOChannelIndex,
    last_updated: Instant
}

impl<T: std::fmt::Display + Clone> SensoryChannelStreamCache<T> {
    
    pub fn new(stream_cache_processor: Box<dyn StreamCacheProcessor<T>>,
               neuron_xyzp_encoder: Box<dyn NeuronXYZPEncoder<T>>,
               channel: CorticalIOChannelIndex,
    ) -> Result<Self, FeagiDataProcessingError> {

        Ok(SensoryChannelStreamCache {
            stream_cache_processor,
            neuron_xyzp_encoder,
            channel,
            last_updated: Instant::now()
        })
        
    }
    
    pub fn update_sensor_value(&mut self, value: T) -> Result<(), FeagiDataProcessingError> {
        _ = self.stream_cache_processor.process_new_input(&value)?;
        self.last_updated = Instant::now();
        Ok(())
    }
    
    pub fn is_more_recent_than_given(&self, time: Instant) -> bool {
        time < self.last_updated
    }
    
    pub fn get_most_recent_sensor_value(&self) -> &T {
        self.stream_cache_processor.get_most_recent_output()
    }
    
    pub fn encode_to_neurons(&self, neuron_xyzp_arrays: &mut NeuronXYZPArrays) -> Result<(), FeagiDataProcessingError> {
        self.neuron_xyzp_encoder.write_neuron_data_single_channel(
            self.stream_cache_processor.get_most_recent_output().clone(),
            neuron_xyzp_arrays,
            self.channel)
    }
    
    pub fn get_cortical_IO_channel_index(&self) -> CorticalIOChannelIndex {
        self.channel
    }
}

// TODO add callback for only on change

pub struct MotorChannelStreamCache<T: std::fmt::Display> {
    stream_cache_processor: Box<dyn StreamCacheProcessor<T>>,
    neuron_xyzp_decoder: Box<dyn NeuronXYZPDecoder<T>>,
    cortical_id: CorticalID,
    channel: CorticalIOChannelIndex,
    last_updated: Instant,
    callbacks_all_bursts: CallBackManager<T>
}

impl<T: std::fmt::Display + Clone> MotorChannelStreamCache<T> {
    
    pub fn new(stream_cache_processor: Box<dyn StreamCacheProcessor<T>>, 
               neuron_xyzp_decoder: Box<dyn NeuronXYZPDecoder<T>>,
               cortical_id: CorticalID,
               channel: CorticalIOChannelIndex) -> Result<Self, FeagiDataProcessingError> {
        
        if ! matches!(cortical_id.get_cortical_type(), CorticalType::Motor(_)) {
            return Err(IODataError::InvalidParameters("Cortical ID must be of a motor cortical area to create a MotorChannelStreamCache!".into()).into())
        }
        
        Ok(MotorChannelStreamCache{
            stream_cache_processor,
            neuron_xyzp_decoder,
            cortical_id,
            channel,
            last_updated: Instant::now(),
            callbacks_all_bursts: CallBackManager::new()
        })
        
    }
    
    pub fn decode_from_neurons(&mut self, neuron_data: &NeuronXYZPArrays) -> Result<&T, FeagiDataProcessingError> {
        let decoded_value: T = self.neuron_xyzp_decoder.read_neuron_data_single_channel(
            neuron_data,
            self.channel
        )?;
        self.last_updated = Instant::now();
        self.stream_cache_processor.process_new_input(&decoded_value)
    }
    
    pub fn is_more_recent_than_given(&self, time: Instant) -> bool {
        time < self.last_updated
    }
    
    pub fn get_most_recent_motor_value(&self) -> &T {
        self.stream_cache_processor.get_most_recent_output()
    }
    
    // TODO allow registering callbacks
}

