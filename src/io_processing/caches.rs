use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::genomic_structures::CorticalType;
use super::device_group_caches::SensorXYZPDeviceGroupCache;


pub struct SensoryIOCache {
    cached: HashMap<CorticalType, SensorXYZPDeviceGroupCache>,
}

impl SensoryIOCache {
    
    pub fn register_sensor(sensor_type: CorticalType)
    
    
        
    
    
    fn register_sensor_type(sensor_type: CorticalType) -> Result<(), FeagiDataProcessingError> {
        let sensor_group_cache: SensorXYZPDeviceGroupCache = SensorXYZPDeviceGroupCache::new(
            sensor_type,
            
        )?;
        
        
    }
        
}