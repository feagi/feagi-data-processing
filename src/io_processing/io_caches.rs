use std::collections::HashMap;
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType, SingleChannelDimensions};
use crate::io_data::IOTypeData;
use crate::io_processing::{SensoryChannelStreamCache, StreamCacheProcessor};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};
use crate::neuron_data::xyzp::{NeuronCoderVariantType, instantiate_encoder_by_type};

pub struct SensorCache {
    channel_caches: HashMap<FullChannelCacheKey, SensoryChannelStreamCache>,
    cortical_area_metadata: HashMap<CorticalAreaMetadataKey, CorticalAreaCacheDetails>,
    agent_key_proxy: HashMap<AccessAgentLookupKey, Vec<FullChannelCacheKey>>
}

impl SensorCache {

    pub fn new() -> SensorCache {
        SensorCache {
            channel_caches: HashMap::new(),
            cortical_area_metadata: HashMap::new(),
            agent_key_proxy: HashMap::new()
        }
    }

    pub fn register_single_cortical_area(&mut self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex, number_supported_channels: u32, channel_dimensions: SingleChannelDimensions)
        -> Result<(), FeagiDataProcessingError> {

        cortical_type.verify_is_sensor()?;
        if self.is_cortical_area_registered(cortical_type, cortical_grouping_index) {
            return Err(IODataError::InvalidParameters(format!("Cortical area of type {:?} of group index {:?} is already registered", cortical_type, cortical_grouping_index)).into());
        }
        if number_supported_channels == 0 {
            return Err(IODataError::InvalidParameters("A cortical area cannot be registered with 0 channels!".into()).into())
        }
        channel_dimensions.verify_restrictions(&cortical_type.try_get_channel_size_boundaries()?)?; // Verify given channel dimensions are sensible for this cortical type
        
        
        
        let cortical_metadata_key = CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index);
        let cortical_id_write_targets: Vec<CorticalID> = vec![cortical_type.try_as_cortical_id(cortical_grouping_index)?]; // Only 1
        let neuron_encoder_type = cortical_type.try_get_coder_type()?;
        let neuron_encoder = instantiate_encoder_by_type(neuron_encoder_type, &cortical_id_write_targets, channel_dimensions)?;
        
        _ = self.cortical_area_metadata.insert(
            cortical_metadata_key,
            CorticalAreaCacheDetails::new(cortical_id_write_targets, number_supported_channels, neuron_encoder)
        );
        Ok(())
    }
    
    pub fn register_segmented_vision_cortical_areas(&mut self, cortical_grouping_index: CorticalGroupingIndex, number_supported_channels: u32)  -> Result<(), FeagiDataProcessingError> {

        // Unique case (TODO: add check for segmented encoder type)
        //if cortical_type == CorticalType::Sensory(SensorCorticalType::VisionCenterGray) && false {
        //    cortical_id_write_targets = CorticalID::create_ordered_cortical_areas_for_segmented_vision(cortical_grouping_index, true).to_vec();
        //}
        Err(FeagiDataProcessingError::NotImplemented)
    }
    

    pub fn register_channel(&mut self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex,
                            channel: CorticalIOChannelIndex, sensory_processor: Box<dyn StreamCacheProcessor + Sync>, should_sensor_allow_sending_stale_data: bool) ->
    Result<(), FeagiDataProcessingError> {

        cortical_type.verify_is_sensor()?;
        let verify_type = cortical_type.verify_valid_io_variant(&sensory_processor.get_output_data_type());
        if verify_type.is_err() { // let's print out a more detailed error in this case
            return Err(IODataError::InvalidParameters(format!("The sensory filter outputs {:?}, which is not allowed for this cortical type {:?}! The only allowed types for this cortical type are '{}'!  (Note the input of the sensor filter does NOT need to match)",
                                                              sensory_processor.get_output_data_type(), cortical_type, cortical_type.get_possible_io_variants().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
            )).into())
        }
        let cortical_area_details =  self.try_get_cortical_area_cache_details(cortical_type, cortical_grouping_index)?;
        if *channel >= cortical_area_details.number_channels {
            return Err( IODataError::InvalidParameters(format!("Unable to set channel index to {} as the channel count for cortical type {:?} group index {:?} is {}",
                                                               *channel, cortical_type, cortical_grouping_index, cortical_area_details.number_channels)).into());
        }
        if self.is_channel_cache_registered(cortical_type, cortical_grouping_index, channel) {
            return Err( IODataError::InvalidParameters(format!("Unable to register sensor cache to already existing Cortical Type {:?}, Group Index {:?}, Channel {:?}!",
                                                               cortical_type, cortical_grouping_index, channel)).into())
        }

        let full_channel_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel);
        let sensory_stream_cache = SensoryChannelStreamCache::new(sensory_processor, channel, should_sensor_allow_sending_stale_data)?;
        _ = self.channel_caches.insert(full_channel_key, sensory_stream_cache);
        Ok(())
    }

    pub fn register_agent_device_index(&mut self, agent_device_index: AgentDeviceIndex, cortical_type: CorticalType,
                                       cortical_grouping_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {

        cortical_type.verify_is_sensor()?;
        _ = self.try_get_channel_cache(cortical_type, cortical_grouping_index, channel)?;

        let full_channel_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel);
        let try_key_vector = self.try_get_mut_agent_proxy_keys(cortical_type, agent_device_index);
        match try_key_vector {
            Ok(key_vector) => {
                // Listing exists, lets expand it
                key_vector.push(full_channel_key)
            }
            Err(_) => {
                // No listing exists, create one
                let new_vector: Vec<FullChannelCacheKey> = vec![full_channel_key];
                _ = self.agent_key_proxy.insert(AccessAgentLookupKey::new(cortical_type, agent_device_index), new_vector);
            }
        }
        Ok(())
    }

    pub fn update_value_by_channel(&mut self, value: IOTypeData, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {
        self.try_update_value(value, cortical_type, cortical_grouping_index, channel)?;
        Ok(())
    }
    
    pub fn update_value_by_agent_device_index(&mut self, value: IOTypeData, cortical_type: CorticalType, agent_device_index: AgentDeviceIndex) -> Result<(), FeagiDataProcessingError> {
        // Due to borrowing restrictions, had to expand "try_get_agent_proxy_keys"
        let channel_keys: &Vec<FullChannelCacheKey> = match self.agent_key_proxy.get(&AccessAgentLookupKey::new(cortical_type, agent_device_index)) {
            Some(keys) => keys,
            None => return Err(IODataError::InvalidParameters(format!("No device registered for cortical type {:?} using agent device index{:?}!", cortical_type, agent_device_index)).into())
        };
        
        match channel_keys.len() {
            0 => {
                return Err(FeagiDataProcessingError::InternalError("Agent Device Index called on mapping with zero elements!".into())); // This should never be possible
            }
            1 => {
                // Most common case, only one mapping
                let channel_key = &channel_keys[0];
                self.try_update_value(value, channel_key.cortical_type, channel_key.cortical_group, channel_key.channel)?;
                Ok(())
            }
            number_keys => {
                // Multiple mappings. In order to save 1 clone operation, we update the values for Number_mapped_keys - 1 with clones, and simply pass the ownership for the last one
                let second_last_index = number_keys - 1;
                for i in 0..second_last_index {
                    // Due to borrowing restrictions, had to expand
                    let channel_key = &channel_keys[i];
                    match self.channel_caches.get_mut(&channel_key) {
                        Some(channel_cache) => {
                            if channel_cache.get_input_data_type() != value.variant() {
                                return Err(IODataError::InvalidParameters(format!("Got value type {:?} when expected type {:?} for Cortical Type {:?}, Group Index {:?}, Channel {:?}!", value.variant(),
                                                                                  channel_cache.get_input_data_type(), channel_key.cortical_type ,channel_key.cortical_group , channel_key.channel)).into());
                            }
                            _ = channel_cache.update_sensor_value(value.clone());
                        }
                        None => {
                            return Err(IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", channel_key.cortical_type ,channel_key.cortical_group , channel_key.channel)).into())
                        }
                    }
                }
                let channel_key = &channel_keys[second_last_index];
                self.try_update_value(value.clone(), channel_key.cortical_type, channel_key.cortical_group, channel_key.channel)?;
                Ok(())
            }
        }
    }

    pub fn encode_to_neurons(&self, past_send_time: Instant, neurons_to_encode_to: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // TODO move to using iter(), I'm using for loops now cause im still a rust scrub
        for cortical_area_details in self.cortical_area_metadata.values() {
            let cortical_id_targets = &cortical_area_details.neuron_data_location_by_cortical_ids;
            let channel_cache_keys = &cortical_area_details.relevant_channel_lookups;
            let neuron_encoder = &cortical_area_details.neuron_encoder;
            for channel_cache_key in channel_cache_keys {
                let sensor_cache = self.channel_caches.get(channel_cache_key).unwrap();
                sensor_cache.encode_to_neurons(neurons_to_encode_to, neuron_encoder)?
            }
        }
        Ok(())

    }


    
    
    fn is_cortical_area_registered(&self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex) -> bool {
        self.cortical_area_metadata.contains_key(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index))
    }

    fn try_get_cortical_area_cache_details(&self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex) -> Result<&CorticalAreaCacheDetails, FeagiDataProcessingError> {
        let result = self.cortical_area_metadata.get(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index));
        match result {
            Some(area_cache_details) => Ok(area_cache_details),
            None => Err(IODataError::InvalidParameters(format!("Cortical Area of Type {:?} of group index {:?} not found!", cortical_type, cortical_grouping_index)).into())
        }
    }



    fn is_channel_cache_registered(&self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex,
                                   channel: CorticalIOChannelIndex) -> bool {
        self.channel_caches.contains_key(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel))
    }

    fn try_get_channel_cache(&self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex,
                                 channel: CorticalIOChannelIndex) -> Result<&SensoryChannelStreamCache, FeagiDataProcessingError> {
        let result = self.channel_caches.get(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel));
        match result {
            Some(channel_stream_cache) => Ok(channel_stream_cache),
            None => Err(IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, channel)).into())
        }
    }

    fn try_get_mut_channel_cache(&mut self, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex,
                                 channel: CorticalIOChannelIndex) -> Result<&mut SensoryChannelStreamCache, FeagiDataProcessingError> {
        let result = self.channel_caches.get_mut(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel));
        match result {
            Some(channel_stream_cache) => Ok(channel_stream_cache),
            None => Err(IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, channel)).into())
        }
    }



    fn is_agent_key_proxy_registered(&self, cortical_type: CorticalType, agent_grouping_index: AgentDeviceIndex) -> bool {
        self.agent_key_proxy.contains_key(&AccessAgentLookupKey::new(cortical_type, agent_grouping_index))
    }

    fn try_get_agent_proxy_keys(&self, cortical_type: CorticalType, agent_grouping_index: AgentDeviceIndex) -> Result<&Vec<FullChannelCacheKey>, FeagiDataProcessingError> {
        let result = self.agent_key_proxy.get(&AccessAgentLookupKey::new(cortical_type, agent_grouping_index));
        match result {
            Some(agent_proxy_keys) => {Ok(agent_proxy_keys)}
            None => Err(IODataError::InvalidParameters(format!("No device registered for cortical type {:?} using agent device index{:?}!", cortical_type, agent_grouping_index)).into())
        }
    }

    fn try_get_mut_agent_proxy_keys(&mut self, cortical_type: CorticalType, agent_grouping_index: AgentDeviceIndex) -> Result<&mut Vec<FullChannelCacheKey>, FeagiDataProcessingError> {
        let result = self.agent_key_proxy.get_mut(&AccessAgentLookupKey::new(cortical_type, agent_grouping_index));
        match result {
            Some(agent_proxy_keys) => {Ok(agent_proxy_keys)}
            None => Err(IODataError::InvalidParameters(format!("No device registered for cortical type {:?} using agent device index{:?}!", cortical_type, agent_grouping_index)).into())
        }
    }

    
    
    fn try_update_value(&mut self, value: IOTypeData, cortical_type: CorticalType, cortical_grouping_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {
        let mut cache = self.try_get_mut_channel_cache(cortical_type, cortical_grouping_index, channel)?;
        if cache.get_input_data_type() != value.variant() {
            return Err(IODataError::InvalidParameters(format!("Got value type {:?} when expected type {:?} for Cortical Type {:?}, Group Index {:?}, Channel {:?}!", value.variant(),
                                                              cache.get_input_data_type(), cortical_type, cortical_grouping_index, channel)).into());
        }
        _ = cache.update_sensor_value(value);
        Ok(())
    }

}





























/// Key needed to get direct access to channel cache
#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct FullChannelCacheKey {
    pub cortical_type: CorticalType,
    pub cortical_group: CorticalGroupingIndex,
    pub channel: CorticalIOChannelIndex,
}

impl FullChannelCacheKey {
    pub fn new(cortical_type: CorticalType, cortical_group: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Self {
        FullChannelCacheKey {
            cortical_type,
            cortical_group,
            channel,
        }
    }
}



#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct CorticalAreaMetadataKey {
    pub cortical_type: CorticalType,
    pub cortical_group: CorticalGroupingIndex,
}

impl CorticalAreaMetadataKey {
    pub fn new(cortical_type: CorticalType, cortical_group: CorticalGroupingIndex) -> Self {
        CorticalAreaMetadataKey {
            cortical_type,
            cortical_group,
        }
    }
}



#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct AccessAgentLookupKey {
    pub cortical_type: CorticalType,
    pub agent_index: AgentDeviceIndex,
}

impl AccessAgentLookupKey {
    pub fn new(cortical_type: CorticalType, agent_index: AgentDeviceIndex) -> Self {
        AccessAgentLookupKey{
            cortical_type,
            agent_index,
        }
    }
}




pub(crate) struct CorticalAreaCacheDetails {
    pub neuron_data_location_by_cortical_ids: Vec<CorticalID>,
    pub relevant_channel_lookups: Vec<FullChannelCacheKey>,
    pub number_channels: u32,
    pub neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync>
}

impl  CorticalAreaCacheDetails {
    pub fn new(neuron_data_location_by_cortical_ids: Vec<CorticalID>, number_channels: u32, neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync>) -> Self {
        CorticalAreaCacheDetails{
            neuron_data_location_by_cortical_ids,
            relevant_channel_lookups: Vec::new(),
            number_channels,
            neuron_encoder
        }

    }
}