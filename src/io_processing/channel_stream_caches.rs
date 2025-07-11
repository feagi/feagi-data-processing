use std::time::{Instant};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::{CallBackManager, StreamCacheProcessor};
use crate::neuron_data::xyzp::{NeuronXYZPArrays, NeuronXYZPDecoder, NeuronXYZPEncoder};

#[derive(Debug, Clone)]
pub struct SensoryChannelStreamCache {
    stream_cache_processor: Box<dyn StreamCacheProcessor>,
    neuron_xyzp_encoder:  Box< dyn NeuronXYZPEncoder>,
    channel: CorticalIOChannelIndex,
    last_updated: Instant
}

impl SensoryChannelStreamCache {
    
    pub fn new(stream_cache_processor: Box<dyn StreamCacheProcessor>,
               neuron_xyzp_encoder: Box<dyn NeuronXYZPEncoder>,
               channel: CorticalIOChannelIndex,
    ) -> Result<Self, FeagiDataProcessingError> {
        
        if stream_cache_processor.get_data_type() != neuron_xyzp_encoder.get_encoded_data_type() {
            return Err(FeagiDataProcessingError::InternalError("Stream Cache Processor and Neuron Encoder do not have matching data types!".into()));
        }
        
        Ok(SensoryChannelStreamCache {
            stream_cache_processor,
            neuron_xyzp_encoder,
            channel,
            last_updated: Instant::now()
        })
        
    }
    
    pub fn update_sensor_value(&mut self, value: IOTypeData) -> Result<(), FeagiDataProcessingError> {
        _ = self.stream_cache_processor.process_new_input(&value)?;
        self.last_updated = Instant::now();
        Ok(())
    }
    
    pub fn is_more_recent_than_given(&self, time: Instant) -> bool {
        time < self.last_updated
    }
    
    pub fn get_most_recent_sensor_value(&self) -> &IOTypeData {
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
    
    pub fn get_data_type(&self) -> IOTypeVariant {
        self.stream_cache_processor.get_data_type()
    }
}

// TODO add callback for only on change

pub struct MotorChannelStreamCache {
    stream_cache_processor: Box<dyn StreamCacheProcessor>,
    neuron_xyzp_decoder: Box<dyn NeuronXYZPDecoder>,
    channel: CorticalIOChannelIndex,
    last_updated: Instant,
    callbacks_all_bursts: CallBackManager
}

impl MotorChannelStreamCache {
    
    pub fn new(stream_cache_processor: Box<dyn StreamCacheProcessor>, 
               neuron_xyzp_decoder: Box<dyn NeuronXYZPDecoder>,
               channel: CorticalIOChannelIndex) -> Result<Self, FeagiDataProcessingError> {

        if stream_cache_processor.get_data_type() != neuron_xyzp_decoder.get_data_type() {
            return Err(FeagiDataProcessingError::InternalError("Stream Cache Processor and Neuron Decoder do not have matching data types!".into()));
        }
        
        Ok(MotorChannelStreamCache{
            stream_cache_processor,
            neuron_xyzp_decoder,
            channel,
            last_updated: Instant::now(),
            callbacks_all_bursts: CallBackManager::new()
        })
        
    }
    
    pub fn decode_from_neurons(&mut self, neuron_data: &NeuronXYZPArrays) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let decoded_value: IOTypeData = self.neuron_xyzp_decoder.read_neuron_data_single_channel(
            neuron_data,
            self.channel
        )?;
        self.last_updated = Instant::now();
        self.stream_cache_processor.process_new_input(&decoded_value)
    }
    
    pub fn is_more_recent_than_given(&self, time: Instant) -> bool {
        time < self.last_updated
    }
    
    pub fn get_most_recent_motor_value(&self) -> &IOTypeData {
        self.stream_cache_processor.get_most_recent_output()
    }

    pub fn get_data_type(&self) -> IOTypeVariant {
        self.stream_cache_processor.get_data_type()
    }
    
    // TODO allow registering callbacks
}

