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


    fn register_cortical_area(&mut self, sensor_cortical_type: SensorCorticalType, cortical_group: CorticalGroupingIndex, number_supported_channels: usize, neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>) -> Result<(), FeagiDataProcessingError> {
        let cortical_type = sensor_cortical_type.into();
        let cortical_metadata = CorticalAreaMetadataKey::new(cortical_type, cortical_group);

        if number_supported_channels == 0 {
            return Err(IODataError::InvalidParameters("A cortical area cannot be registered with 0 channels!".into()).into())
        }
        if self.cortical_area_metadata.contains_key(&cortical_metadata) {
            return Err(FeagiDataProcessingError::InternalError("cortical area already registered!".into()).into())
        }


    }

    fn register_cache_on_channel(cortical_type: CorticalType, cortical_group: CorticalGroupingIndex,
                                 channel_index: CorticalIOChannelIndex, cache_processors: Vec<Box<dyn StreamCacheProcessor + Sync + Send>>,
                                 should_allow_sending_stale_data: bool) -> Result<(), FeagiDataProcessingError> {



    }


    //endregion
    
    
    
}


















struct CorticalAreaCacheDetails {
    relevant_channel_lookups: Vec<FullChannelCacheKey>,
    number_channels: u32,
    neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>
}

impl  CorticalAreaCacheDetails {
    pub(crate) fn new(number_channels: u32, neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>) -> Self {
        CorticalAreaCacheDetails{
            relevant_channel_lookups: Vec::new(),
            number_channels,
            neuron_encoder
        }

    }
}