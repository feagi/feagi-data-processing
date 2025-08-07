//! High-level sensor data caching and management for FEAGI neural processing.
//!
//! This module provides the main sensor cache system that manages multiple cortical
//! areas, channels, and agent device mappings. It serves as the primary interface
//! for registering sensory inputs and converting them to neural representations.

use std::collections::HashMap;
use std::time::Instant;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalIOChannelIndex, CorticalType, SensorCorticalType, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::{StreamCacheProcessor};
use crate::io_processing::channel_stream_caches::SensoryChannelStreamCache;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};

/// High-level sensor data cache managing multiple cortical areas and channels.
///
/// `SensorCache` is the main coordination point for all sensory input processing
/// in FEAGI. It manages the registration of cortical areas, individual channels,
/// and agent device mappings, while providing unified access to cached sensor
/// data and neural encoding capabilities.
///
/// # Architecture
///
/// The cache operates at three organizational levels:
/// - **Cortical Areas**: Groups of related channels with shared properties
/// - **Individual Channels**: Specific sensor streams within areas
/// - **Agent Devices**: External device interfaces mapped to channels
///
/// # Key Features
///
/// - **Multi-level Registration**: Register areas, channels, and device mappings
/// - **Type Safety**: Validates data types throughout the processing chain
/// - **Flexible Access**: Update and retrieve data by device channel or device index
/// - **Neural Integration**: Direct encoding to neural representations
/// - **Metadata Management**: Tracks area properties and encoder configurations
///
/// # Usage Workflow
///
/// ## 1. Register Cortical Areas
/// ## 2. Register Individual Channels
/// ## 3. Map Agent Devices (Optional)
/// ## 4. Update data on Channels as it come in
/// ## 5. Neural Encoding
pub struct SensorCache {
    channel_caches: HashMap<FullChannelCacheKey, SensoryChannelStreamCache>,
    cortical_area_metadata: HashMap<CorticalAreaMetadataKey, CorticalAreaCacheDetails>,
    agent_key_proxy: HashMap<AccessAgentLookupKey, Vec<FullChannelCacheKey>>
}

impl SensorCache {

    /// Creates a new empty sensor cache.
    ///
    /// Initializes all internal data structures for managing cortical areas,
    /// channels, and agent device mappings. The cache starts empty and requires
    /// explicit registration of cortical areas before use.
    ///
    /// # Returns
    ///
    /// A new `SensorCache` instance ready for registration and data processing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_processing::SensorCache;
    ///
    /// let mut cache = SensorCache::new();
    /// // Cache is now ready for cortical area registration
    /// ```
    pub fn new() -> SensorCache {
        SensorCache {
            channel_caches: HashMap::new(),
            cortical_area_metadata: HashMap::new(),
            agent_key_proxy: HashMap::new()
        }
    }

    /// Registers a single cortical area with specified characteristics.
    ///
    /// Creates a new cortical area that can contain multiple sensor channels.
    /// This is the first step in setting up sensor processing - the area must
    /// be registered before individual channels can be added.
    ///
    /// # Arguments
    ///
    /// * `cortical_sensor_type` - The type of sensory processing (vision, audio, etc.)
    /// * `cortical_grouping_index` - Unique identifier for this area instance
    /// * `number_supported_channels` - Maximum number of channels this area can contain
    /// * `channel_dimensions` - 3D dimensions for each device channel in this area
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Area successfully registered
    /// * `Err(FeagiDataProcessingError)` - Registration failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The area type and grouping index combination is already registered
    /// - `number_supported_channels` is zero
    /// - `channel_dimensions` are outside the valid range for this sensor type
    /// - Neural encoder instantiation fails
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
    
    /// Registers segmented vision cortical areas for advanced visual processing.
    ///
    /// **Note: This function is not yet implemented.**
    ///
    /// This method is intended for registering specialized vision processing
    /// areas that handle segmented visual input, typically for complex
    /// visual processing scenarios.
    ///
    /// # Arguments
    ///
    /// * `cortical_grouping_index` - Grouping identifier for the vision areas
    /// * `number_supported_channels` - Number of channels per segmented area
    ///
    /// # Returns
    ///
    /// Currently, always returns `Err(FeagiDataProcessingError::NotImplemented)`
    ///
    /// # Future Implementation
    ///
    /// Will register multiple related cortical areas for segmented vision
    /// processing, with specialized encoders for handling visual segments.
    pub fn register_segmented_vision_cortical_areas(&mut self, cortical_grouping_index: CorticalGroupingIndex, number_supported_channels: u32 )  -> Result<(), FeagiDataProcessingError> {

        // Unique case (TODO: add check for segmented encoder type)
        //if cortical_type == CorticalType::Sensory(SensorCorticalType::VisionCenterGray) && false {
        //    cortical_id_write_targets = CorticalID::create_ordered_cortical_areas_for_segmented_vision(cortical_grouping_index, true).to_vec();
        //}
        Err(FeagiDataProcessingError::NotImplemented)
    }
    

    /// Registers a single sensor device channel within a cortical area.
    ///
    /// Creates an individual sensor device channel with its own processing pipeline
    /// within a previously registered cortical area. Each device channel handles
    /// a specific sensor data stream and applies its own processing chain.
    ///
    /// # Arguments
    ///
    /// * `cortical_sensor_type` - Type of the parent cortical area
    /// * `cortical_grouping_index` - Grouping index of the parent cortical area
    /// * `channel` - Channel index within the cortical area (must be < number_supported_channels)
    /// * `sensory_processors` - Chain of processors to apply to incoming data
    /// * `should_sensor_allow_sending_stale_data` - Whether to allow cached data when no fresh updates
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Channel successfully registered
    /// * `Err(FeagiDataProcessingError)` - Registration failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent cortical area is not registered
    /// - Channel index exceeds the area's device channel capacity
    /// - Channel is already registered
    /// - Stream cache processor setup fails
    pub fn register_single_channel(&mut self, cortical_sensor_type: SensorCorticalType, cortical_grouping_index: CorticalGroupingIndex,
                                   device_channel: CorticalIOChannelIndex, sensory_processors: Vec<Box<dyn StreamCacheProcessor + Sync + Send>>, should_sensor_allow_sending_stale_data: bool) ->
    Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
        let cortical_area_details =  match self.cortical_area_metadata.get(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index)) {
            Some(cache_details) => cache_details,
            None => return Err(IODataError::InvalidParameters(format!("Cortical Area of Type {:?} of group index {:?} not found!", cortical_type, cortical_grouping_index)).into())
        };
        
        if *device_channel >= cortical_area_details.number_channels {
            return Err( IODataError::InvalidParameters(format!("Unable to set device channel index to {} as the device channel count for cortical type {:?} group index {:?} is {}",
                                                               *device_channel, cortical_type, cortical_grouping_index, cortical_area_details.number_channels)).into());
        }
        if self.channel_caches.contains_key(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, device_channel)) {
            return Err( IODataError::InvalidParameters(format!("Unable to register sensor cache to already existing Cortical Type {:?}, Group Index {:?}, Channel {:?}!",
                                                               cortical_type, cortical_grouping_index, device_channel)).into())
        }

        let full_channel_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_grouping_index, device_channel);
        let sensory_stream_cache = SensoryChannelStreamCache::new(sensory_processors, device_channel, should_sensor_allow_sending_stale_data)?;
        let cortical_area_details =  self.cortical_area_metadata.get_mut(&CorticalAreaMetadataKey::new(cortical_type, cortical_grouping_index)).unwrap();
        _ = cortical_area_details.relevant_channel_lookups.push(full_channel_key.clone());
        _ = self.channel_caches.insert(full_channel_key, sensory_stream_cache);
        Ok(())
    }

    /// Maps an agent device index to a specific sensor channel.
    ///
    /// Creates a mapping that allows external devices to send data using
    /// agent device indices rather than explicit cortical coordinates.
    /// Multiple channels can be mapped to the same device index if they
    /// accept the same data type.
    ///
    /// # Arguments
    ///
    /// * `agent_device_index` - External device identifier
    /// * `cortical_sensor_type` - Target cortical area type
    /// * `cortical_grouping_index` - Target cortical area grouping
    /// * `channel` - Target device channel within the cortical area
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Device mapping successfully registered
    /// * `Err(FeagiDataProcessingError)` - Mapping failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Target device channel is not registered
    /// - Device index already maps to channels with incompatible data types
    ///
    /// # Data Type Validation
    ///
    /// When mapping multiple channels to the same device index, all channels
    /// must accept the same input data type. This ensures consistent behavior
    /// when broadcasting data from the device.
    pub fn register_agent_device_index(&mut self, agent_device_index: AgentDeviceIndex, cortical_sensor_type: SensorCorticalType,
                                       cortical_grouping_index: CorticalGroupingIndex, device_channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
        _ = self.channel_caches.get(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, device_channel))
            .ok_or_else(|| IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, device_channel)))?;
        
        let full_channel_key: FullChannelCacheKey = FullChannelCacheKey::new(cortical_type, cortical_grouping_index, device_channel);
        let try_key_vector = self.agent_key_proxy.get_mut(&AccessAgentLookupKey::new(cortical_type, agent_device_index));
        
        match try_key_vector {
            Some(key_vector) => {
                // There already is a mapping. Verify the input data types match
                let new_checking_cache = self.channel_caches.get(&full_channel_key).unwrap();
                let first_key = key_vector.first().unwrap();
                let first_checking_cache = self.channel_caches.get(first_key).unwrap();
                if new_checking_cache.get_input_data_type() != first_checking_cache.get_input_data_type() {
                    return Err(IODataError::InvalidParameters(format!("Cannot to the same Agent Device Index {} that already contains a device channel accepting {} another device channel that accepts {}! Types must match!",
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

    /// Updates sensor data for a specific device channel using explicit coordinates.
    ///
    /// Directly updates a sensor device channel with new data using the full cortical
    /// coordinate specification. The data is validated against the channel's
    /// expected input type before processing.
    ///
    /// # Arguments
    ///
    /// * `value` - New sensor data to process and cache
    /// * `cortical_sensor_type` - Type of the target cortical area
    /// * `cortical_grouping_index` - Grouping index of the target cortical area
    /// * `channel` - Specific device channel within the cortical area
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Data successfully updated and processed
    /// * `Err(FeagiDataProcessingError)` - Update failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Target device channel is not registered
    /// - Data type doesn't match channel's expected input type
    /// - Processing chain fails
    pub fn update_value_by_channel(&mut self, value: IOTypeData, cortical_sensor_type: SensorCorticalType, cortical_grouping_index: CorticalGroupingIndex, device_channel: CorticalIOChannelIndex) -> Result<(), FeagiDataProcessingError> {

        let cortical_type = cortical_sensor_type.into();
        let channel_cache = match self.channel_caches.get_mut(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, device_channel)) {
            Some(channel_stream_cache) => channel_stream_cache,
            None => return Err(IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, device_channel)).into())
        };
        
        if channel_cache.get_input_data_type() != IOTypeVariant::from(&value) {
            return Err(IODataError::InvalidParameters(format!("Got value type {:?} when expected type {:?} for Cortical Type {:?}, Group Index {:?}, Channel {:?}!", IOTypeVariant::from(&value),
                                                              channel_cache.get_input_data_type(), cortical_type, cortical_grouping_index, device_channel)).into());
        }
        _ = channel_cache.update_sensor_value(value);
        Ok(())
    }
    
    /// Updates sensor data using an agent device index.
    ///
    /// Updates all channels mapped to the specified agent device index with
    /// the provided data. If multiple channels are mapped to the same device,
    /// the data is efficiently broadcast to all of them.
    ///
    /// # Arguments
    ///
    /// * `value` - New sensor data to process and cache
    /// * `cortical_sensor_type` - Type of cortical area for the device mapping
    /// * `agent_device_index` - External device identifier
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Data successfully updated for all mapped channels
    /// * `Err(FeagiDataProcessingError)` - Update failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No channels are mapped to the specified device index
    /// - Internal mapping data is corrupted (zero elements)
    ///
    /// # Efficiency Notes
    ///
    /// When multiple channels are mapped to the same device:
    /// - The last device channel receives the original data (no clone)
    /// - All other channels receive cloned copies
    /// - This minimizes memory allocation overhead
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
    
    /// Retrieves the latest processed value from a specific channel.
    ///
    /// Returns the most recent sensor data that has been processed through
    /// the channel's processing pipeline. This provides access to the current
    /// state of a sensor without triggering any updates.
    ///
    /// # Arguments
    ///
    /// * `cortical_sensor_type` - Type of the target cortical area
    /// * `cortical_grouping_index` - Grouping index of the target cortical area
    /// * `channel` - Specific device channel within the cortical area
    ///
    /// # Returns
    ///
    /// * `Ok(&IOTypeData)` - Reference to the latest processed sensor data
    /// * `Err(FeagiDataProcessingError)` - Channel not found
    ///
    /// # Errors
    ///
    /// Returns an error if the specified device channel is not registered.
    pub fn get_latest_value_by_channel(&mut self, cortical_sensor_type: SensorCorticalType, cortical_grouping_index: CorticalGroupingIndex, device_channel: CorticalIOChannelIndex) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let cortical_type = cortical_sensor_type.into();
        let channel_cache = match self.channel_caches.get(&FullChannelCacheKey::new(cortical_type, cortical_grouping_index, device_channel)) {
            Some(channel_stream_cache) => channel_stream_cache,
            None => return Err(IODataError::InvalidParameters(format!("Unable to find Cortical Type {:?}, Group Index {:?}, Channel {:?}!", cortical_type, cortical_grouping_index, device_channel)).into())
        };
        Ok(channel_cache.get_most_recent_sensor_value())
    }

    /// Encodes all cached sensor data into neural representations.
    ///
    /// Converts the current state of all registered sensor channels into
    /// neural activity patterns suitable for FEAGI's neural processing system.
    /// This is typically called during each processing cycle to provide
    /// sensory input to the neural brain.
    ///
    /// # Arguments
    ///
    /// * `past_send_time` - Timestamp reference for staleness checking (currently unused)
    /// * `neurons_to_encode_to` - Target neural data structure to populate
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All sensor data successfully encoded
    /// * `Err(FeagiDataProcessingError)` - Encoding failed for one or more channels
    ///
    /// # Errors
    ///
    /// Returns an error if any channel's neural encoding fails, typically due to:
    /// - Incompatible data types between cache and encoder
    /// - Neural data structure capacity limits
    /// - Encoder-specific processing failures
    ///
    /// # Processing Details
    ///
    /// The method iterates through all registered cortical areas and their
    /// associated channels, applying the appropriate neural encoder for each
    /// area type. Each encoder converts the processed sensor data into the
    /// neural representation format expected by FEAGI.
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



/// Key needed to get direct access to device channel cache
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