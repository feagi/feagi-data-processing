// Caches all streaming data relevant to a cortical type

use std::collections::HashMap;
use std::hash::Hash;
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::{StreamCacheProcessor};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};
use super::channel_stream_caches::SensoryChannelStreamCache;

// TODO can we collapse the lookup for agent device indexes into a single hashmap lookup? It should be possible

//region helper structs
#[derive(Debug, Hash, Eq, PartialEq)]
struct SensorKey {
    pub cortical_grouping_index: CorticalGroupingIndex,
    pub cortical_io_channel_index: CorticalIOChannelIndex,
}

impl SensorKey {
    pub fn new(cortical_grouping_index: CorticalGroupingIndex, cortical_io_channel_index: CorticalIOChannelIndex) -> Self {
        SensorKey { cortical_grouping_index, cortical_io_channel_index }
    }
}

#[derive(Debug)]
struct SensorChannelMappedResult {
    pub sensory_channel_stream_cache: SensoryChannelStreamCache,
    pub cortical_id: CorticalID
}

impl SensorChannelMappedResult {
    pub fn new(sensory_channel_stream_cache: SensoryChannelStreamCache, cortical_id: CorticalID) -> Self {
        SensorChannelMappedResult { sensory_channel_stream_cache, cortical_id }
    }
}
//endregion


pub struct SensorXYZPDeviceGroupCache {
    representing_cortical_type: CorticalType,
    channel_mappings: HashMap<SensorKey, SensorChannelMappedResult>,
    agent_device_mapping: HashMap<AgentDeviceIndex, Vec<SensorKey>>,
    neuron_encoder: Box<dyn NeuronXYZPEncoder>,
    encoder_cortical_group_to_channel_fast_lookup: HashMap<CorticalGroupingIndex, Vec<CorticalGroupingIndex>>
}

impl SensorXYZPDeviceGroupCache {
    
    pub fn new(cortical_type: CorticalType, neuron_encoder: Box<dyn NeuronXYZPEncoder>) -> Result<Self, FeagiDataProcessingError> {
        cortical_type.verify_is_sensor()?;
        cortical_type.verify_valid_io_variant()
        
        Ok(SensorXYZPDeviceGroupCache{
            representing_cortical_type: cortical_type,
            channel_mappings: HashMap::new(),
            agent_device_mapping: HashMap::new(),
        })
    }
    
    pub fn register_sensory_channel(&mut self, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex, sensory_processor: Box<dyn StreamCacheProcessor>, should_sensor_allow_sending_stale_data: bool) -> Result<(), FeagiDataProcessingError> {
        // We cannot check here if the channel index is too large!
        
        if self.is_channel_mapped(cortical_grouping, channel) {
            return Err(IODataError::InvalidParameters(format!("Channel mapping of cortical group '{}' and channel index '{}' is already registered!", cortical_grouping.to_string(), channel.to_string())).into())
        }
        let verify_type = self.representing_cortical_type.verify_valid_io_variant(&sensory_processor.get_output_data_type());
        if verify_type.is_err() { // let's print out a more detailed error in this case
            return Err(IODataError::InvalidParameters(format!("The sensory filter outputs {}, which is not allowed for this cortical type {}! The only allowed types for this cortical type are '{}'!  (Note the input of the sensor filter does NOT need to match)",
                                                              sensory_processor.get_output_data_type().to_string(), self.representing_cortical_type.to_string(), self.representing_cortical_type.get_possible_io_variants().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
            )).into())
        }
        
        // TODO check if any image segment processing is done on none center types!

        let cortical_id = CorticalID::try_from_cortical_type(&self.representing_cortical_type, cortical_grouping)?;
        let sensor_key = SensorKey::new(cortical_grouping, channel);
        let sensory_stream_cache = SensoryChannelStreamCache::new(sensory_processor, channel, should_sensor_allow_sending_stale_data)?;
        let sensor_mapped_result = SensorChannelMappedResult::new(sensory_stream_cache, cortical_id);
        _ = self.channel_mappings.insert(sensor_key, sensor_mapped_result);
        Ok(())
    }
    
    pub fn register_agent_device_index_to_sensory_channel(&mut self, agent_index: AgentDeviceIndex, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex)  -> Result<(), FeagiDataProcessingError> {
        if !self.is_channel_mapped(cortical_grouping, channel) {
            return Err(IODataError::InvalidParameters(format!("No sensory channel registration of channel index {} under cortical group {} exists for agent device index registration!",
                                                              channel.to_string(), cortical_grouping.to_string())).into())
        }
        
        if !self.is_agent_mapped(agent_index) {
            self.agent_device_mapping.insert(agent_index, vec![SensorKey::new(cortical_grouping, channel)]);
            return Ok(())
        }
        
        let vec: &mut Vec<SensorKey> = self.agent_device_mapping.get_mut(&agent_index).unwrap();
        vec.push(SensorKey::new(cortical_grouping, channel));
        Ok(())
    }
    
    pub fn update_value_by_channel(&mut self, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex, value: IOTypeData) -> Result<(), FeagiDataProcessingError> {
        self.update_value(value, &SensorKey::new(cortical_grouping, channel))?;
        Ok(())
    }
    
    pub fn update_value_by_agent_index(&mut self, agent_device_index: AgentDeviceIndex, value: IOTypeData) -> Result<(), FeagiDataProcessingError> {
        // Get immutable reference to the sensor keys to check existence and length
        let sensor_keys = match self.agent_device_mapping.get(&agent_device_index) {
            Some(keys) => keys,
            None => {
                return Err(IODataError::InvalidParameters(format!("No agent device index  registration of {} exists!",
                                                                  agent_device_index.to_string())).into())
            }
        };
        
        // We cannot make use of "update_value" here as we cannot borrow self mutably twice, so we expanded that function here.
        match sensor_keys.len() {
            0 => {
                return Err(FeagiDataProcessingError::InternalError("Agent Device Index called on mapping with zero elements!".into()))
            }
            1 => {
                // Most common case where there is a 1 to 1 mapping
                let sensor_key = &sensor_keys[0];
                match self.channel_mappings.get_mut(sensor_key) {
                    Some(mapped_channels) => {
                        _ = mapped_channels.sensory_channel_stream_cache.update_sensor_value(value)?;
                        return Ok(());
                    }
                    None => {
                        return Err(FeagiDataProcessingError::InternalError(format!("No sensory channel registration of channel index {} under cortical grouping {} exists!",
                                                                          sensor_key.cortical_io_channel_index.to_string(), sensor_key.cortical_grouping_index.to_string())).into())
                    }
                }
            }
            number_mappings => {
                // For multiple mappings, we need to clone the value for all but the last
                let last_index = number_mappings - 1;
                for i in 0..number_mappings {
                    let sensor_key = &sensor_keys[i];

                    match self.channel_mappings.get_mut(sensor_key) {
                        Some(mapped_channels) => {
                            _ = mapped_channels.sensory_channel_stream_cache.update_sensor_value(value.clone())?;
                        }
                        None => {
                            return Err(FeagiDataProcessingError::InternalError(format!("No sensory channel registration of channel index {} under cortical grouping {} exists!",
                                                                              sensor_key.cortical_io_channel_index.to_string(), sensor_key.cortical_grouping_index.to_string())).into())
                        }
                    }
                    // Me when I have no option but to use clone https://media1.tenor.com/m/ChT4Gyw2N-0AAAAd/anger-duck.gif
                }
                
                // Handle the last one without cloning the value
                let sensor_key = &sensor_keys[last_index];
                match self.channel_mappings.get_mut(sensor_key) {
                    Some(mapped_channels) => {
                        _ = mapped_channels.sensory_channel_stream_cache.update_sensor_value(value)?;
                        return Ok(());
                    }
                    None => {
                        return Err(FeagiDataProcessingError::InternalError(format!("No sensory channel registration of channel index {} under cortical grouping {} exists!",
                                                                          sensor_key.cortical_io_channel_index.to_string(), sensor_key.cortical_grouping_index.to_string())).into())
                    }
                }    
            }
        }
    }
    

    pub fn encode_new_data_to_neurons(&self, past_send_time: Instant, neurons_to_encode_to: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // TODO move to using iter(), I'm using for loops now cause im still a rust scrub
        for stream_processor_and_id in self.channel_mappings.values() {
            if !stream_processor_and_id.sensory_channel_stream_cache.should_push_new_value(past_send_time) {
                continue;
            }
            stream_processor_and_id.sensory_channel_stream_cache.encode_to_neurons(
                neurons_to_encode_to,
                
            )?;
            
            
            
            //stream_processor.
        };

        Ok(())
    }

    
    fn is_channel_mapped(&self, cortical_group: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> bool {
        self.channel_mappings.get(&SensorKey::new(cortical_group, channel)).is_some()
    }
    
    fn is_agent_mapped(&self, agent_index: AgentDeviceIndex) -> bool {
        self.agent_device_mapping.get(&agent_index).is_some()
    }
    
    fn update_value(&mut self, value: IOTypeData, sensor_key: &SensorKey) -> Result<(), FeagiDataProcessingError> {
        match self.channel_mappings.get_mut(sensor_key) {
            Some(mapped_channels) => {
                _ = mapped_channels.sensory_channel_stream_cache.update_sensor_value(value)?;
                return Ok(());
            }
            None => {
                return Err(IODataError::InvalidParameters(format!("No sensory channel registration of channel index {} under cortical grouping {} exists!",
                                                                  sensor_key.cortical_io_channel_index.to_string(), sensor_key.cortical_grouping_index.to_string())).into())
            }
        }
    }
    
    

}