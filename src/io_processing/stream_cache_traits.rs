use std::fmt;
use crate::genomic_structures::{AgentDeviceIndex, CorticalID, CorticalIOChannelIndex, CorticalGroupingIndex};
use crate::neuron_data::xyzp::{NeuronXYZPDecoder, NeuronXYZPEncoder};

pub trait StreamXYZPCache<T: fmt::Display> : fmt::Display {
    fn get_cached_data(&self) -> &T;
    fn get_cortical_grouping_index(&self) -> &CorticalGroupingIndex;
    fn get_agent_device_index(&self) -> &AgentDeviceIndex;
    fn get_cortical_io_channel_index(&self) -> &CorticalIOChannelIndex;
    fn get_cortical_area_id(&self) -> &CorticalID;
}

pub trait StreamXYZPSensoryCaches<T: fmt::Display> : StreamXYZPCache<T> {
    
    fn new(encoder: dyn NeuronXYZPEncoder<T>, )
    
    fn replace_cached_data(&mut self, data: &T);
    
    
}