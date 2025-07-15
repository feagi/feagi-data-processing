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
struct SensorMappedResult {
    pub sensory_channel_stream_cache: SensoryChannelStreamCache,
    pub cortical_id: CorticalID
}

impl SensorMappedResult {
    pub fn new(sensory_channel_stream_cache: SensoryChannelStreamCache, cortical_id: CorticalID) -> Self {
        SensorMappedResult { sensory_channel_stream_cache, cortical_id }
    }
}



pub struct SensorXYZPDeviceGroupCache {
    representing_cortical_type: CorticalType,
    channel_mappings: HashMap<SensorKey, SensorMappedResult>,
    agent_device_mapping: HashMap<AgentDeviceIndex, Vec<SensorKey>>
    
}

impl SensorXYZPDeviceGroupCache {
    
    pub fn new(cortical_type: CorticalType) -> Result<Self, FeagiDataProcessingError> {
        cortical_type.verify_is_sensor()?;
        
        Ok(SensorXYZPDeviceGroupCache{
            representing_cortical_type: cortical_type,
            channel_mappings: HashMap::new(),
            agent_device_mapping: HashMap::new(),
        })
    }
    
    pub fn register_sensory_channel(&mut self, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex, sensory_processor: Box<dyn StreamCacheProcessor>) -> Result<(), FeagiDataProcessingError> {
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
        let sensory_stream_cache = SensoryChannelStreamCache::new(sensory_processor, channel)?;
        let sensor_mapped_result = SensorMappedResult::new(sensory_stream_cache, cortical_id);
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
        let check_sensor_keys: Option<&mut Vec<SensorKey>> = self.agent_device_mapping.get_mut(&agent_device_index);
        
        if check_sensor_keys.is_none() {
            return Err(IODataError::InvalidParameters(format!("No agent device index  registration of {} exists!",
                                                              agent_device_index.to_string())).into())
        }
        let possible_sensor_mappings: &mut Vec<SensorKey> = check_sensor_keys.unwrap();
        match possible_sensor_mappings.len() {
            0 => {
                return Err(FeagiDataProcessingError::InternalError("Agent Device Index called on mapping with zero elements!".into()))
            }
            1 => {
                // Most common case where there is a 1 to 1 mapping. We do this to avoid cloning the value as we can just transfer the ownership instead
                self.update_value(value, &possible_sensor_mappings[0])?;
                return Ok(());
            }
            (number_mappings_from_agent_index) => {
                // In order to use 1 less clone, we will iterate over the vector except for the last value, at which then we simply pass in the value's ownership
                let last_agent_index_index = number_mappings_from_agent_index - 1;
                for agent_index_index in 0..last_agent_index_index {
                    let cortical_id_channel = &possible_sensor_mappings[agent_index_index];
                    self.update_value(value.clone(), cortical_id_channel)?; // We cannot avoid cloning. Each filter needs to own the value for the processing it may do
                    // Me when I have no option but to use clone https://media1.tenor.com/m/ChT4Gyw2N-0AAAAd/anger-duck.gif
                }
                let cortical_id_channel = &possible_sensor_mappings[last_agent_index_index];
                self.update_value(value, cortical_id_channel)?;
                return Ok(());
            }
        }
    }
    
    pub fn encode_new_data_to_neurons(&self, current_time: Instant, neurons_to_encode_to: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        for stream_processor_and_id in self.channel_mappings.values() {
            let mut stream_processor = &stream_processor_and_id.0;
            let cortical_id = &stream_processor_and_id.1;
            
            if stream_processor.is_set_to_update_only_on_change() && stream_processor.is_value_more_recent_than_given(current_time) {
                continue; // If our value is more recent, it means the data is old (already sent) and thus shouldn't be sent again
            }

            stream_processor.
            
            
        };
    }
    
    
    fn is_channel_mapped(&self, cortical_group: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> bool {
        self.channel_mappings.get(&SensorKey::new(cortical_group, channel)).is_some()
    }
    
    fn is_agent_mapped(&self, agent_index: AgentDeviceIndex) -> bool {
        self.agent_device_mapping.get(&agent_index).is_some()
    }
    
    fn update_value(&mut self, value: IOTypeData, sensor_key: &SensorKey) -> Result<(), FeagiDataProcessingError> {
        let mapping = self.channel_mappings.get_mut(sensor_key);
        if mapping.is_none() {
            return Err(IODataError::InvalidParameters(format!("No sensory channel registration of channel index {} under cortical grouping {} exists!",
                                                              sensor_key.cortical_io_channel_index.to_string(), sensor_key.cortical_grouping_index.to_string())).into())
        }
        let mapped = mapping.unwrap();
        _ = mapped.sensory_channel_stream_cache.update_sensor_value(value)?;
        Ok(())
    }
    
    

}