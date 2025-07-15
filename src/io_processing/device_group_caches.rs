// Caches all streaming data relevant to a cortical type

use std::collections::HashMap;
use std::hash::Hash;
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::{StreamCacheFilter};
use crate::neuron_data::xyzp::NeuronXYZPEncoder;

// TODO can we collapse the lookup for agent device indexes into a single hashmap lookup? It should be possible

type SensorKey = (CorticalGroupingIndex, CorticalIOChannelIndex);

pub struct SensorXYZPDeviceGroupCache {
    representing_cortical_type: CorticalType,
    channel_mappings: HashMap<SensorKey, (Box<dyn StreamCacheFilter>, CorticalID)>,
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
    
    pub fn register_sensory_channel(&mut self, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex, sensory_filter: Box<dyn StreamCacheFilter>) -> Result<(), FeagiDataProcessingError> {
        // We cannot check here if the channel index is too large!
        
        if self.is_channel_mapped(cortical_grouping, channel) {
            return Err(IODataError::InvalidParameters(format!("Channel mapping of cortical group '{}' and channel index '{}' is already registered!", cortical_grouping.to_string(), channel.to_string())).into())
        }
        let verify_type = self.representing_cortical_type.verify_valid_io_variant(&sensory_filter.get_output_data_type());
        if verify_type.is_err() { // let's print out a more detailed error in this case
            return Err(IODataError::InvalidParameters(format!("The sensory filter outputs {}, which is not allowed for this cortical type {}! The only allowed types for this cortical type are '{}'!  (Note the input of the sensor filter does NOT need to match)",
                                                              sensory_filter.get_output_data_type().to_string(), self.representing_cortical_type.to_string(), self.representing_cortical_type.get_possible_io_variants().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
            )).into())
        }
        
        // TODO check if any image segment processing is done on none center types!

        let cortical_id = CorticalID::try_from_cortical_type(&self.representing_cortical_type, cortical_grouping)?;
        let sensor_key = (cortical_grouping, channel);
        _ = self.channel_mappings.insert(sensor_key, (sensory_filter, cortical_id));
        Ok(())
        
    }
    
    pub fn register_agent_device_index_to_sensory_channel(&mut self, agent_index: AgentDeviceIndex, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex)  -> Result<(), FeagiDataProcessingError> {
        if !self.is_channel_mapped(cortical_grouping, channel) {
            return Err(IODataError::InvalidParameters(format!("No sensory channel registration of channel index {} under cortical group {} exists for agent device index registration!",
                                                              channel.to_string(), cortical_grouping.to_string())).into())
        }
        
        if !self.is_agent_mapped(agent_index) {
            self.agent_device_mapping.insert(agent_index, vec![(cortical_grouping, channel)]);
            return Ok(())
        }
        
        let vec: &mut Vec<SensorKey> = self.agent_device_mapping.get_mut(&agent_index).unwrap();
        vec.push((cortical_grouping, channel));
        Ok(())
    }
    
    pub fn update_value_by_channel(&mut self, cortical_grouping: CorticalGroupingIndex, channel: CorticalIOChannelIndex, value: IOTypeData) -> Result<(), FeagiDataProcessingError> {
        let mapping = self.channel_mappings.get_mut(&(cortical_grouping, channel));
        if mapping.is_none() {
            return Err(IODataError::InvalidParameters(format!("No sensory channel registration of channel index {} under cortical grouping {} exists!",
                                                              channel.to_string(), cortical_grouping.to_string())).into())
        }
        let filter = mapping.unwrap();
        _ = filter.0.process_new_input(value)?;
        Ok(())
    }
    
    pub fn update_value_by_agent_index(&mut self, agent_device_index: AgentDeviceIndex, value: IOTypeData) -> Result<(), FeagiDataProcessingError> {
        let mapping = self.agent_device_mapping.get_mut(&agent_device_index);
        if mapping.is_none() {
            return Err(IODataError::InvalidParameters(format!("No agent device index  registration of {} exists!",
                                                              agent_device_index.to_string())).into())
        }
        
        let mappings_to_cortical_grouping_and_channel: &mut Vec<(CorticalGroupingIndex, CorticalIOChannelIndex)> = mapping.unwrap();
        match mappings_to_cortical_grouping_and_channel.len() {
            0 => {
                return Err(FeagiDataProcessingError::InternalError("Agent Device Index called on mapping with zero elements!".into()))
            }
            1 => {
                // Most common case where there is a 1 to 1 mapping. We do this to avoid cloning the value as we can just transfer the ownership instead
                let cortical_id_channel = mappings_to_cortical_grouping_and_channel[0];
                let filter = self.channel_mappings.get_mut(&cortical_id_channel);
                if filter.is_none() {
                    return Err(FeagiDataProcessingError::InternalError(format!("failed to locate initialized agent device index {}!", agent_device_index.to_string())))
                }
                let filter = filter.unwrap();
                _ = filter.0.process_new_input(value)?;
                return Ok(());
            }
            (number_mappings_from_agent_index) => {
                // In order to use 1 less clone, we will iterate over the vector except for the last value, at which then we simply pass in the value's ownership
                let last_agent_index_index = number_mappings_from_agent_index - 1;
                for agent_index_index in 0..last_agent_index_index {
                    let cortical_id_channel = mappings_to_cortical_grouping_and_channel[agent_index_index];
                    let filter = self.channel_mappings.get_mut(&cortical_id_channel);
                    if filter.is_none() {
                        return Err(FeagiDataProcessingError::InternalError(format!("failed to locate initialized agent device index {}!", agent_device_index.to_string())))
                    }
                    let filter = filter.unwrap();
                    _ = filter.0.process_new_input(value.clone())?; // We cannot avoid this. Each filter needs to own the value for the processing it may do
                    // Me when I have no option but to use clone https://media1.tenor.com/m/ChT4Gyw2N-0AAAAd/anger-duck.gif
                }
                let cortical_id_channel = mappings_to_cortical_grouping_and_channel[last_agent_index_index];
                let filter = self.channel_mappings.get_mut(&cortical_id_channel);
                if filter.is_none() {
                    return Err(FeagiDataProcessingError::InternalError(format!("failed to locate initialized agent device index {}!", agent_device_index.to_string())))
                }
                let filter = filter.unwrap();
                _ = filter.0.process_new_input(value)?;
                return Ok(());
            }
        }
    }
    
    
    fn is_channel_mapped(&self, cortical_group: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> bool {
        self.channel_mappings.get(&(cortical_group, channel)).is_some()
    }
    
    fn is_agent_mapped(&self, agent_index: AgentDeviceIndex) -> bool {
        self.agent_device_mapping.get(&agent_index).is_some()
    }
    
    
    

}