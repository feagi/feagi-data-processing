use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{CorticalGroupingIndex, CorticalIOChannelIndex, CorticalType, SensorCorticalType, SingleChannelDimensions};
use crate::io_processing::caches::hashmap_helpers::{FullChannelCacheKey, CorticalAreaMetadataKey, AccessAgentLookupKey};
use crate::io_processing::sensory_channel_stream_cache::SensoryChannelStreamCache;
use crate::io_processing::StreamCacheProcessor;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};

pub struct SensorCache {
    channel_caches: HashMap<FullChannelCacheKey, SensoryChannelStreamCache>, // (cortical type, grouping index, channel) -> sensory data cache, the main lookup
    cortical_area_metadata: HashMap<CorticalAreaMetadataKey, CorticalAreaCacheDetails>, // (cortical type, grouping index) -> (Vec<FullChannelCacheKey>, number_channels, neuron_encoder), defines all channel caches for a cortical area, and its neuron encoder
    agent_key_proxy: HashMap<AccessAgentLookupKey, Vec<FullChannelCacheKey>>, // (CorticalType, AgentDeviceIndex) -> Vec<FullChannelCacheKey>, allows users to map any channel of a cortical type to an agent device ID
    neuron_data: CorticalMappedXYZPNeuronData // cached neuron data
}

impl SensorCache {
    pub fn new() -> SensorCache {
        SensorCache {
            channel_caches: HashMap::new(),
            cortical_area_metadata: HashMap::new(),
            agent_key_proxy: HashMap::new(),
            neuron_data: CorticalMappedXYZPNeuronData::new(),
        }
    }














    //region Internal Functions


    //region by type registration
    
    fn register_cortical_
    
    
    
    //endregion
    
    
    
    
    
    
    fn register_cortical_area_and_channels(&mut self, sensor_cortical_type: SensorCorticalType, cortical_group: CorticalGroupingIndex, 
                              number_supported_channels: u32, neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>, 
                              default_processor_chain: Vec<Box<dyn StreamCacheProcessor + Sync + Send>>,
                              allow_stale_data: bool) -> Result<(), FeagiDataProcessingError> {
        
        let cortical_type = sensor_cortical_type.into();
        let cortical_metadata = CorticalAreaMetadataKey::new(cortical_type, cortical_group);
        
        if number_supported_channels == 0 {
            return Err(IODataError::InvalidParameters("A cortical area cannot be registered with 0 channels!".into()).into())
        }
        if self.cortical_area_metadata.contains_key(&cortical_metadata) {
            return Err(FeagiDataProcessingError::InternalError("Cortical area already registered!".into()).into())
        }
        
        
        let mut cache_keys: Vec<FullChannelCacheKey> = Vec::with_capacity(number_supported_channels as usize);
        for i in 0..number_supported_channels {
            
            let channel: CorticalIOChannelIndex = i.into();
            let sensor_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_group, channel);
            let sensor_cache: SensoryChannelStreamCache = SensoryChannelStreamCache::new(
                default_processor_chain.clone(),
                channel,
                allow_stale_data
            )?;
            
            _ = self.channel_caches.insert(sensor_key.clone(), sensor_cache);
            cache_keys.push(sensor_key);
        }
        
        
        let cortical_cache_details = CorticalAreaCacheDetails::new(cache_keys, number_supported_channels, neuron_encoder);
        _ = self.cortical_area_metadata.insert(cortical_metadata, cortical_cache_details);
        
        Ok(())
    }
    
    
    


    //endregion
    
    
    
}






struct CorticalAreaCacheDetails {
    relevant_channel_lookups: Vec<FullChannelCacheKey>,
    number_channels: u32,
    neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>
}

impl  CorticalAreaCacheDetails {
    pub(crate) fn new(relevant_channel_lookups: Vec<FullChannelCacheKey>, number_channels: u32, neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>) -> Self {
        CorticalAreaCacheDetails{
            relevant_channel_lookups,
            number_channels,
            neuron_encoder
        }

    }
}