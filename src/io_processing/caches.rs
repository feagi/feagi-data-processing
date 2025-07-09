use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError, IODataError};
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
    device_lookup: HashMap<AgentDeviceIndex, Vec<SensoryChannelStreamCache>>,
    // Stream caches use location references to find caches in device_lookup
    stream_caches: HashMap<CacheKey, CacheLocation>
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
        io_cache: SensoryChannelStreamCache
    ) -> Result<(), FeagiDataProcessingError> {
        
        match cortical_type {
            CorticalType::Custom => { return Err(IODataError::InvalidParameters("Cannot register a Custom Cortical Area as a Sensor!".into()).into()) }
            CorticalType::Core(_) => {return Err(IODataError::InvalidParameters("Cannot register a Core Cortical Area as a Sensor!".into()).into())}
            CorticalType::Memory => {return Err(IODataError::InvalidParameters("Cannot register a Memory Cortical Area as a Sensor!".into()).into())}
            CorticalType::Motor(_) => {return Err(IODataError::InvalidParameters("Cannot register a Motor Cortical Area as a Sensor!".into()).into())}
            CorticalType::Sensory(_) => {} // continue
        }
        
        return Err(FeagiDataProcessingError::NotImplemented)
        
        
        
    }
        
        
}