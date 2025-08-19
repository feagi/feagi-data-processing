
use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType, SensorCorticalType, SingleChannelDimensions};
use crate::io_data::image_descriptors::{ImageFrameProperties, GazeProperties, SegmentedImageFrameProperties};
use crate::io_data::{ImageFrame, ImageFrameSegmentator, ImageFrameTransformer, SegmentedImageFrame};
use crate::io_processing::caches::hashmap_helpers::{FullChannelCacheKey, CorticalAreaMetadataKey, AccessAgentLookupKey};
use crate::io_processing::processors::{IdentitySegmentedImageFrameProcessor, ImageFrameSegmentatorProcessor, ImageFrameTransformerProcessor, LinearScaleTo0And1Processor};
use crate::io_processing::sensory_channel_stream_cache::SensoryChannelStreamCache;
use crate::io_processing::StreamCacheProcessor;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};
use crate::neuron_data::xyzp::encoders::{F32LinearNeuronXYZPEncoder, ImageFrameNeuronXYZPEncoder, SegmentedImageFrameNeuronXYZPEncoder};

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
    
    //region Registration

    //region macro

    pub fn register_cortical_group_for_proximity(&mut self, cortical_group: CorticalGroupingIndex,
                                                 number_of_channels: usize,
                                                 allow_stale_data: bool,
                                                 neuron_resolution: usize,
                                                 lower_bound: f32,
                                                 upper_bound: f32) -> Result<(), FeagiDataProcessingError> {

        self.register_cortical_area_f32_normalized_0_to_1_linear(SensorCorticalType::Proximity,
                                                                 cortical_group, number_of_channels,
                                                                 neuron_resolution, lower_bound,
                                                                 upper_bound, allow_stale_data)
    }

    //endregion

    //region Custom Calls

    pub fn register_cortical_group_for_image_camera(&mut self, cortical_group: CorticalGroupingIndex,
                                                    number_of_channels: usize, allow_stale_data: bool,
                                                    input_image_properties: ImageFrameProperties,
                                                    output_image_properties: ImageFrameProperties) -> Result<(), FeagiDataProcessingError> {

        // TODO instead of hard coding, maybe expose and use an optional var in python to select camera location?
        // We always pick center for this
        self.register_cortical_area_image_frame(SensorCorticalType::ImageCameraCenter,
                                                cortical_group, number_of_channels,
                                                input_image_properties, output_image_properties,
                                                allow_stale_data)
    }

    pub fn register_cortical_group_for_image_camera_with_peripheral(&mut self, cortical_group: CorticalGroupingIndex,
                                                                    number_of_channels: usize, allow_stale_data: bool,
                                                                    input_image_properties: ImageFrameProperties,
                                                                    output_image_properties: SegmentedImageFrameProperties,
                                                                    segmentation_center_properties: GazeProperties) -> Result<(), FeagiDataProcessingError> {
        
        let sensor_cortical_type = SensorCorticalType::ImageCameraCenter;
        self.verify_number_channels(number_of_channels)?;
        let cortical_ids = SegmentedImageFrame::create_ordered_cortical_ids_for_segmented_vision(cortical_group);
        for cortical_id in &cortical_ids {
            let cortical_type = cortical_id.get_cortical_type();
            let cortical_metadata = CorticalAreaMetadataKey::new(cortical_type, cortical_group);
            if self.cortical_area_metadata.contains_key(&cortical_metadata) {
                return Err(FeagiDataProcessingError::InternalError("Cortical area already registered!".into()).into())
            }
        }; // ensure no cortical ID is used already
        
        let segmentator = ImageFrameSegmentator::new(input_image_properties, output_image_properties, segmentation_center_properties)?;
        let neuron_encoder = Box::new(SegmentedImageFrameNeuronXYZPEncoder::new(cortical_ids, output_image_properties)?);
        let mut processors: Vec<Vec<Box<dyn StreamCacheProcessor + Sync + Send>>> = Vec::with_capacity(number_of_channels);
        for i in 0..number_of_channels {
            processors.push(vec![Box::new(ImageFrameSegmentatorProcessor::new(input_image_properties, output_image_properties, segmentator.clone()))]);
        };
        
        self.register_cortical_area_and_channels(sensor_cortical_type, cortical_group, neuron_encoder, processors, allow_stale_data)?;
        Ok(())


    }

    //endregion
    
    //endregion
    
    
    //region Agent Index Functions

    fn register_agent_device_index(&mut self, agent_device_index: AgentDeviceIndex, cortical_sensor_type: SensorCorticalType,
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
    
    //endregion






    //region Internal Functions
    
    //region by type registration
    
    fn register_cortical_area_f32_normalized_0_to_1_linear(&mut self, sensor_cortical_type: SensorCorticalType, 
                                                           cortical_group: CorticalGroupingIndex,
                                                           number_of_channels: usize,
                                                           neuron_resolution: usize,
                                                           lower_bound: f32,
                                                           upper_bound: f32,
                                                           allow_stale_data: bool) -> Result<(), FeagiDataProcessingError> {
        //TODO template should allow checking for input data type
        
        if neuron_resolution == 0 {
            return Err(IODataError::InvalidParameters("Unable to define a neuron resolution of 0!".into()).into())
        }
        if upper_bound <= lower_bound {
            return Err(IODataError::InvalidParameters("Upper bound must not be less than lower bound!".into()).into())
        }
        self.verify_number_channels(number_of_channels)?;
        
        
        let cortical_id = CorticalID::new_sensor_cortical_area_id(sensor_cortical_type, cortical_group)?;
        let neuron_encoder = Box::new(F32LinearNeuronXYZPEncoder::new(cortical_id, neuron_resolution as u32)?);
        let mut processors: Vec<Vec<Box<dyn StreamCacheProcessor + Sync + Send>>> = Vec::with_capacity(number_of_channels);
        for i in 0..number_of_channels {
            processors.push(vec![Box::new(LinearScaleTo0And1Processor::new(lower_bound, upper_bound, 0.0)?)]);
        };
        
        self.register_cortical_area_and_channels(sensor_cortical_type, cortical_group, neuron_encoder, processors, allow_stale_data)?;
        Ok(())
    }
    
    fn register_cortical_area_image_frame(&mut self, sensor_cortical_type: SensorCorticalType, 
                                          cortical_group: CorticalGroupingIndex, 
                                          number_of_channels: usize, 
                                          input_image_properties: ImageFrameProperties,
                                          output_image_properties: ImageFrameProperties, 
                                          allow_stale_data: bool)  -> Result<(), FeagiDataProcessingError> {

        //TODO template should allow checking for input data type
        self.verify_number_channels(number_of_channels)?;
        
        let image_transformer_definition = ImageFrameTransformer::new_from_input_output_properties(&input_image_properties, &output_image_properties)?;
        
        let cortical_id = CorticalID::new_sensor_cortical_area_id(sensor_cortical_type, cortical_group)?;
        let neuron_encoder = Box::new(ImageFrameNeuronXYZPEncoder::new(cortical_id, &output_image_properties)?);
        let mut processors: Vec<Vec<Box<dyn StreamCacheProcessor + Sync + Send>>> = Vec::with_capacity(number_of_channels);
        for i in 0..number_of_channels {
            processors.push(vec![Box::new(ImageFrameTransformerProcessor::new(image_transformer_definition)?)]);
        };
        
        self.register_cortical_area_and_channels(sensor_cortical_type, cortical_group, neuron_encoder, processors, allow_stale_data)?;
        Ok(())
    }
    

    
    
    
    //endregion
    
    
    
    
    
    //region Generic Caching
    
    fn register_cortical_area_and_channels(&mut self, sensor_cortical_type: SensorCorticalType, cortical_group: CorticalGroupingIndex,
                                           neuron_encoder: Box<dyn NeuronXYZPEncoder + Sync + Send>,
                                           mut initial_processor_chains: Vec<Vec<Box<dyn StreamCacheProcessor + Sync + Send>>>,
                                           allow_stale_data: bool) -> Result<(), FeagiDataProcessingError> {
        // NOTE: initial_processor_chains is a vector of vectors, meaning each channel gets a vector of processors
        
        let number_supported_channels = initial_processor_chains.len() as u32;
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
                initial_processor_chains.pop().unwrap(),
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
    
    
    
    
    
    fn verify_number_channels(&self, number_of_channels: usize) -> Result<(), FeagiDataProcessingError> {
        if number_of_channels == 0 {
            return Err(IODataError::InvalidParameters("Number of channels must not be zero!".into()).into())
        }
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