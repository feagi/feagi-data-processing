use std::collections::HashMap;
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalIOChannelIndex, CorticalType, SensorCorticalType, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::{StreamCacheProcessor};
use crate::io_processing::channel_stream_caches::SensoryChannelStreamCache;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};

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

    pub fn register_single_cortical_area(&mut self, cortical_sensor_type: SensorCorticalType, cortical_grouping_index: CorticalGroupingIndex, number_supported_channels: u32, channel_dimensions: SingleChannelDimensions)
        -> Result<(), FeagiDataProcessingError> {
        
        let cortical_type = cortical_sensor_type.into();
        if self.cortical_area_metadata.contains_key(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index)) {
            return Err(IODataError::InvalidParameters(format!("Cortical area of type {:?} of group index {:?} is already registered", cortical_type, cortical_grouping_index)).into());
        }
        if number_supported_channels == 0 {
            return Err(IODataError::InvalidParameters("A cortical area cannot be registered with 0 channels!".into()).into())
        }
        
        let acceptable_channel_dimension_range = cortical_type.try_get_channel_size_boundaries()?;
        acceptable_channel_dimension_range.verify_within_range(&channel_dimensions)?;
        
        
        let cortical_metadata_key = CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index);
        let cortical_id = cortical_type.to_cortical_id(cortical_grouping_index)?;
        let neuron_encoder_type = cortical_type.try_get_coder_type()?;
        let neuron_encoder = neuron_encoder_type.instantiate_single_ipu_encoder(&cortical_id, &channel_dimensions)?;
        
        _ = self.cortical_area_metadata.insert(
            cortical_metadata_key,
            CorticalAreaCacheDetails::new(number_supported_channels, neuron_encoder)
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
    

    pub fn register_single_channel(&mut self, cortical_sensor_type: SensorCorticalType, cortical_grouping_index: CorticalGroupingIndex,
                            channel: CorticalIOChannelIndex, sensory_processors: Vec<Box<dyn StreamCacheProcessor + Sync + Send>>, should_sensor_allow_sending_stale_data: bool) ->
    Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
        let cortical_area_details =  match self.cortical_area_metadata.get(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index)) {
            Some(cache_details) => cache_details,
            None => return Err(IODataError::InvalidParameters(format!("Cortical Area of Type {:?} of group index {:?} not found!", cortical_type, cortical_grouping_index)).into())
        };
        
        if *channel >= cortical_area_details.number_channels {
            return Err( IODataError::InvalidParameters(format!("Unable to set channel index to {} as the channel count for cortical type {:?} group index {:?} is {}",
                                                               *channel, cortical_type, cortical_grouping_index, cortical_area_details.number_channels)).into());
        }
        if self.channel_caches.contains_key(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel)) {
            return Err( IODataError::InvalidParameters(format!("Unable to register sensor cache to already existing Cortical Type {:?}, Group Index {:?}, Channel {:?}!",
                                                               cortical_type, cortical_grouping_index, channel)).into())
        }

        let full_channel_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel);
        let sensory_stream_cache = SensoryChannelStreamCache::new(sensory_processors, channel, should_sensor_allow_sending_stale_data)?;
        let cortical_area_details =  self.cortical_area_metadata.get_mut(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index)).unwrap();
        _ = cortical_area_details.relevant_channel_lookups.push(full_channel_key.clone());
        _ = self.channel_caches.insert(full_channel_key, sensory_stream_cache);
        Ok(())
    }

    pub fn register_agent_device_index(&mut self, agent_device_index: AgentDeviceIndex, cortical_sensor_type: SensorCorticalType,
                                       cortical_grouping_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
        _ = self.channel_caches.get(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel))
            .ok_or_else(|| IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, channel)))?;
        
        let full_channel_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel);
        let try_key_vector = self.agent_key_proxy.get_mut(&AccessAgentLookupKey::new(cortical_type, agent_device_index));
        
        match try_key_vector {
            Some(key_vector) => {
                // There already is a mapping. Verify the input data types match
                let new_checking_cache = self.channel_caches.get(&full_channel_key).unwrap();
                let first_key = key_vector.first().unwrap();
                let first_checking_cache = self.channel_caches.get(first_key).unwrap();
                if new_checking_cache.get_input_data_type() != first_checking_cache.get_input_data_type() {
                    return Err(IODataError::InvalidParameters(format!("Cannot to the same Agent Device Index {} that already contains a channel accepting {} another channel that accepts {}! Types must match!",
                                                                      agent_device_index, first_checking_cache.get_input_data_type(), new_checking_cache.get_input_data_type())).into())
                }
                
                key_vector.push(full_channel_key)
            }
            None => {
                // No listing exists, create one
                let new_vector: Vec<FullChannelCacheKey> = vec![full_channel_key];
                _ = self.agent_key_proxy.insert(AccessAgentLookupKey::new(cortical_type, agent_device_index), new_vector);
            }
        }
        Ok(())
    }

    pub fn update_value_by_channel(&mut self, value: IOTypeData, cortical_sensor_type: SensorCorticalType, cortical_grouping_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
        let channel_cache = match self.channel_caches.get_mut(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, channel)) {
            Some(channel_stream_cache) => channel_stream_cache,
            None => return Err(IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, channel)).into())
        };
        
        if channel_cache.get_input_data_type() != IOTypeVariant::from(&value) {
            return Err(IODataError::InvalidParameters(format!("Got value type {:?} when expected type {:?} for Cortical Type {:?}, Group Index {:?}, Channel {:?}!", IOTypeVariant::from(&value),
                                                              channel_cache.get_input_data_type(), cortical_type, cortical_grouping_index, channel)).into());
        }
        _ = channel_cache.update_sensor_value(value);
        Ok(())
    }
    
    pub fn update_value_by_agent_device_index(&mut self, value: IOTypeData, cortical_sensor_type: SensorCorticalType, agent_device_index: AgentDeviceIndex) -> Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
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
                let stream_cache = self.channel_caches.get_mut(&channel_key).unwrap();
                _ = stream_cache.update_sensor_value(value);
                Ok(())
            }
            number_keys => {
                // Multiple mappings. In order to save 1 clone operation, we update the values for Number_mapped_keys - 1 with clones, and simply pass the ownership for the last one
                let second_last_index = number_keys - 1;
                for i in 0..second_last_index {
                    let channel_key = &channel_keys[i];
                    let stream_cache = self.channel_caches.get_mut(&channel_key).unwrap();
                    _ = stream_cache.update_sensor_value(value.clone());
                }
                // The last one
                let channel_key = &channel_keys[second_last_index];
                let stream_cache = self.channel_caches.get_mut(&channel_key).unwrap();
                _ = stream_cache.update_sensor_value(value);
                Ok(())
            }
        }
    }

    pub fn encode_to_neurons(&self, past_send_time: Instant, neurons_to_encode_to: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // TODO move to using iter(), I'm using for loops now cause im still a rust scrub
        for cortical_area_details in self.cortical_area_metadata.values() {
            let channel_cache_keys = &cortical_area_details.relevant_channel_lookups;
            let neuron_encoder = &cortical_area_details.neuron_encoder;
            for channel_cache_key in channel_cache_keys {
                let sensor_cache = self.channel_caches.get(channel_cache_key).unwrap();
                sensor_cache.encode_to_neurons(neurons_to_encode_to, neuron_encoder)?
            }
        }
        Ok(())

    }
    
}



/// Key needed to get direct access to channel cache
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct FullChannelCacheKey {
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
struct CorticalAreaMetadataKey {
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
struct AccessAgentLookupKey {
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



struct CorticalAreaCacheDetails {
    pub relevant_channel_lookups: Vec<FullChannelCacheKey>,
    pub number_channels: u32,
    pub neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>
}

impl  CorticalAreaCacheDetails {
    pub fn new(number_channels: u32, neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>) -> Self {
        CorticalAreaCacheDetails{
            relevant_channel_lookups: Vec::new(),
            number_channels,
            neuron_encoder
        }

    }
}