use std::time::{Instant};
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalIOChannelIndex};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::{StreamCacheProcessor};
use crate::io_processing::stream_cache_processors::ProcessorRunner;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData};
use crate::neuron_data::xyzp::NeuronXYZPEncoder;

// Per channel cache

#[derive(Debug)]
pub struct SensoryChannelStreamCache { 
    processor_runner: ProcessorRunner,
    channel: CorticalIOChannelIndex,
    last_updated: Instant,
    should_allow_sending_stale_data: bool,
}

impl SensoryChannelStreamCache {
    
    pub fn new(cache_processors: Vec<Box<dyn StreamCacheProcessor>>,
               channel: CorticalIOChannelIndex,
               should_allow_sending_stale_data: bool
                ) -> Result<Self, FeagiDataProcessingError> {
        
        let processor_runner = ProcessorRunner::new(cache_processors)?;
        Ok(SensoryChannelStreamCache {
            processor_runner,
            channel,
            last_updated: Instant::now(),
            should_allow_sending_stale_data: should_allow_sending_stale_data
        })
    }
    
    pub fn update_sensor_value(&mut self, value: IOTypeData) -> Result<(), FeagiDataProcessingError> {
        _ = self.processor_runner.update_value(&value)?;
        self.last_updated = Instant::now();
        Ok(())
    }
    
    // True 
    pub fn should_push_new_value(&self, past_push_time: Instant) -> bool {
        self.should_allow_sending_stale_data || past_push_time < self.last_updated
    }
    
    pub fn get_most_recent_sensor_value(&self) -> &IOTypeData {
        self.processor_runner.get_most_recent_output()
    }
    
    pub fn encode_to_neurons(&self, cortical_mapped_neuron_data: &mut CorticalMappedXYZPNeuronData, encoder: &Box<dyn NeuronXYZPEncoder + Sync + Send>) -> Result<(), FeagiDataProcessingError> {
        encoder.write_neuron_data_single_channel(self.get_most_recent_sensor_value(), self.channel, cortical_mapped_neuron_data)
    }
    
    pub fn get_cortical_io_channel_index(&self) -> CorticalIOChannelIndex {
        self.channel
    }

    pub fn get_input_data_type(&self) -> IOTypeVariant {
        self.processor_runner.get_input_data_type()
    }

    pub fn get_output_data_type(&self) -> IOTypeVariant {
        self.processor_runner.get_output_data_type()
    }
}


/*
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

    pub fn get_input_data_type(&self) -> IOTypeVariant {
        self.stream_cache_processor.get_input_data_type()
    }

    pub fn get_output_data_type(&self) -> IOTypeVariant {
        self.stream_cache_processor.get_output_data_type()
    }
    
    // TODO allow registering callbacks
}

 */

