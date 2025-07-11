// Caches all streaming data relevant to an cortical type

use std::collections::HashMap;
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType, SingleChannelDimensions};
use crate::io_data::IOTypeVariant;
use crate::io_processing::StreamCacheProcessor;
use crate::neuron_data::xyzp::NeuronXYZPEncoder;

pub struct SensorXYZPDeviceGroupCache {
    representing_cortical_type: CorticalType,
    per_cortical_area_neuron_encoder_update_time_channel_count: HashMap<CorticalID, (Box<dyn NeuronXYZPEncoder>, Instant, u32)>,
    device_mapping: HashMap<AgentDeviceIndex, (CorticalID, CorticalIOChannelIndex, Instant)>,
    per_channel_processors: HashMap<AgentDeviceIndex, Box<dyn StreamCacheProcessor>>, // TODO doesnt this not allow the use of agent device indexes to multiple channels?
    total_channels_across_all_areas: u32,
    data_type_encoded_to_neurons: IOTypeVariant
}

impl SensorXYZPDeviceGroupCache {
    
    pub fn does_sensor_cortical_area_exist(&self, cortical_index: CorticalGroupingIndex) -> Result<bool, FeagiDataProcessingError> {
        let target_cortical_id: CorticalID = CorticalID::try_from_cortical_type(&self.representing_cortical_type, cortical_index)?;
        Ok(self.per_cortical_area_neuron_encoder_update_time_channel_count.contains_key(&target_cortical_id))
    }
    
    pub fn register_sensor_cortical_area(&mut self, cortical_index: CorticalGroupingIndex, neuron_encoder: Box<dyn NeuronXYZPEncoder>, number_channels: usize) -> Result<(), FeagiDataProcessingError> {
        let target_cortical_id: CorticalID = CorticalID::try_from_cortical_type(&self.representing_cortical_type, cortical_index)?;
        if !self.per_cortical_area_neuron_encoder_update_time_channel_count.contains_key(&target_cortical_id) {
            return Err(IODataError::InvalidParameters(format!("Unable to register sensor to to already registered cortical ID '{}'!", &target_cortical_id.to_string())).into());
        }
        if neuron_encoder.get_encoded_data_type() != self.data_type_encoded_to_neurons {
            return Err(IODataError::InvalidParameters(format!("Cortical Type '{}' only accepts '{}' encoders! Given '{}' encoder is not valid!",
                                                              self.representing_cortical_type.to_string(), self.data_type_encoded_to_neurons.to_string(), neuron_encoder.get_encoded_data_type().to_string())).into());
        }
        if number_channels == 0 {
            return Err(IODataError::InvalidParameters("Cannot register sensor with zero channels!".into()).into());
        }
        
        self.per_cortical_area_neuron_encoder_update_time_channel_count.insert(target_cortical_id, (neuron_encoder, Instant::now(), number_channels));
        Ok(())
    }
    
    pub fn register_sensor_device(&mut self,
                           cortical_index: CorticalGroupingIndex,
                           channel_index: CorticalIOChannelIndex,
                           agent_device_index: AgentDeviceIndex,       
                           stream_cache_processor: Box<dyn StreamCacheProcessor>,
                           neuron_xyzp_encoder: Box<dyn NeuronXYZPEncoder>) -> Result<(), FeagiDataProcessingError> {
        
        let target_cortical_id: CorticalID = CorticalID::try_from_cortical_type(&self.representing_cortical_type, cortical_index)?;
        if !self.per_cortical_area_neuron_encoder_update_time_channel_count.contains_key(&target_cortical_id) {
            return Err(IODataError::InvalidParameters(format!("Unable to register sensor to non-registered cortical ID '{}'!", &target_cortical_id.to_string())).into());
        }
        if *channel_index >= self.per_cortical_area_neuron_encoder_update_time_channel_count.get(&target_cortical_id).unwrap().2 {
            return Err(IODataError::InvalidParameters(format!("Requested Channel index {} is greater than max available'!", *channel_index)).into());
        }
        if self.per_channel_processors.contains_key(&agent_device_index) {
            return Err(IODataError::InvalidParameters(format!("Agent Device Index {} already mapped to '!", *channel_index)).into());
        }
        
        
    }
    
    
    
    
}