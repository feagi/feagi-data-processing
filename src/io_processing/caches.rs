use std::collections::HashMap;
use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{AgentDeviceIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_data::IOTypeData;
use crate::io_processing::SensoryChannelStreamCache;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub cortical_id: CorticalID,
    pub channel_index: CorticalIOChannelIndex,
}

#[derive(Debug, Clone, Copy)]
pub struct CacheLocation {
    pub device_index: AgentDeviceIndex,
    pub vec_index: usize,
}

pub struct SensoryIOCache {
    // Device lookup stores the actual caches directly
    device_lookup: HashMap<AgentDeviceIndex, Vec<Box<dyn SensoryChannelStreamCache<IOTypeData>>>>,
    // Stream caches use location references to find caches in device_lookup
    stream_caches: HashMap<CacheKey, CacheLocation>,
}

impl SensoryIOCache {
    pub fn new() -> Self {
        SensoryIOCache {
            device_lookup: HashMap::new(),
            stream_caches: HashMap::new(),
        }
    }
    
    pub fn register_sensory_device(
        &mut self, 
        cortical_type: CorticalType, 
        local_device: AgentDeviceIndex,
        cortical_index: CorticalIOChannelIndex, 
        io_cache: Box<dyn SensoryChannelStreamCache<IOTypeData>>
    ) -> Result<(), FeagiDataProcessingError> {
        
        
        
        
    }
        
        
}