use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::io_processing::StreamCacheFilter;
use super::device_group_caches::SensorXYZPDeviceGroupCache;


pub struct SensoryIOCache {
    cached: HashMap<CorticalType, SensorXYZPDeviceGroupCache>,
}

impl SensoryIOCache {
    pub fn register_sensor(&mut self, sensor_type: CorticalType, cortical_group_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex, sensory_filter: Box<dyn StreamCacheFilter>) -> Result<(), FeagiDataProcessingError> {
        sensor_type.verify_is_sensor()?;
        
        if !self.cached.contains_key(&sensor_type) {
            self.register_sensor_type(sensor_type);
        }

        let mut sensor_type_cached = self.cached.get_mut(&sensor_type).unwrap();
        sensor_type_cached.register_sensory_channel(cortical_group_index, channel, sensory_filter)?;
        Ok(())
    }

    pub fn register_agent_device_index_to_sensor(&mut self, sensor_type: CorticalType, cortical_group_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex, agent_device_index: AgentDeviceIndex) -> Result<(), FeagiDataProcessingError> {
        sensor_type.verify_is_sensor()?;
        let mut sensor_type_cached = self.try_get_sensor_type_cache_mut(&sensor_type)?;
        sensor_type_cached.register_agent_device_index_to_sensory_channel(agent_device_index, cortical_group_index, channel)
    }

    pub fn update_sensor_value(&mut self, value: IOTypeData, sensor_type: CorticalType, cortical_group_index: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {
        let mut sensor_type_cache = self.try_get_sensor_type_cache_mut(&sensor_type)?;
        sensor_type_cache.update_value_by_channel(cortical_group_index, channel, value)?; // Value type checking occurs at the processor level
        Ok(())
    }
    
    pub fn update_sensor_value_agent_index(&mut self, value: IOTypeData, sensor_type: CorticalType, agent_device_index: AgentDeviceIndex) -> Result<(), FeagiDataProcessingError> {
        let mut sensor_type_cache = self.try_get_sensor_type_cache_mut(&sensor_type)?;
        sensor_type_cache.update_value_by_agent_index(agent_device_index, value)?; // Value type checking occurs at the processor level
        Ok(())
    }


    fn register_sensor_type(&mut self, sensor_type: CorticalType) -> Result<(), FeagiDataProcessingError> {
        if self.cached.contains_key(&sensor_type) {
            return Err(FeagiDataProcessingError::InternalError("Cortical Type already Exists!".into()));
        }
        let sensor_group_cache: SensorXYZPDeviceGroupCache = SensorXYZPDeviceGroupCache::new(
            sensor_type
        )?;
        self.cached.insert(sensor_type, sensor_group_cache);
        Ok(())
    }

    fn try_get_sensor_type_cache_mut(&mut self, sensor_type: &CorticalType) -> Result<&mut SensorXYZPDeviceGroupCache, FeagiDataProcessingError> {
        let result = self.cached.get_mut(sensor_type);
        if result.is_none() {
            return Err(IODataError::InvalidParameters(format!("sensor type {:?} does not exist!", sensor_type)).into())
        }
        Ok(result.unwrap())
    }
        
}