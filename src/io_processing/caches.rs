use std::collections::HashMap;
use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{AgentDeviceIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_processing::SensoryChannelStreamCache;

// TODO prevent modification when running

pub struct SensoryIOCache<'a> {
    stream_caches: HashMap<CorticalID, HashMap<CorticalIOChannelIndex, >>,
    device_lookup: HashMap<AgentDeviceIndex, Vec<&'a AgentDeviceIndex>>,
}

impl<'a> SensoryIOCache<'a> {
    pub fn new() -> Self {
        SensoryIOCache {
            stream_caches: HashMap::new(),
            device_lookup: HashMap::new(),
        }
    }
    
    pub fn register_sensory_device(&mut self, cortical_type: CorticalType, local_device: &AgentDeviceIndex,
    cortical_index: CorticalIOChannelIndex, io_cache: SensoryChannelStreamCache) -> Result<(), FeagiDataProcessingError> {
        if cortical_index != io_cache.
        
        
    }
}