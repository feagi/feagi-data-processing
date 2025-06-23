use std::collections::HashMap;
use crate::data_types::neuron_data::NeuronTranslator;
use super::IOCacheWorker;

pub type GroupIndex = usize;
pub type ChannelIndex = usize;

pub type ChannelDeviceMapping<T> = HashMap<ChannelIndex, dyn IOCacheWorker<T>>;

pub struct InputDeviceGrouping<T> {
    mapping: ChannelDeviceMapping<T>,
    neuron_translator: Box<dyn NeuronTranslator>,
}