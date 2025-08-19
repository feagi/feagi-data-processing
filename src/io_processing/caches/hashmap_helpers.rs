use crate::genomic_structures::{AgentDeviceIndex, CorticalGroupingIndex, CorticalIOChannelIndex, CorticalType};
use crate::neuron_data::xyzp::NeuronXYZPEncoder;

/// Key needed to get direct access to device channel cache
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct FullChannelCacheKey {
    pub(crate) cortical_type: CorticalType,
    pub(crate) cortical_group: CorticalGroupingIndex,
    pub(crate) channel: CorticalIOChannelIndex,
}

impl FullChannelCacheKey {
    pub(crate) fn new(cortical_type: CorticalType, cortical_group: CorticalGroupingIndex, channel: CorticalIOChannelIndex) -> Self {
        FullChannelCacheKey {
            cortical_type,
            cortical_group,
            channel,
        }
    }
}



#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct CorticalAreaMetadataKey {
    pub(crate) cortical_type: CorticalType,
    pub(crate) cortical_group: CorticalGroupingIndex,
}

impl CorticalAreaMetadataKey {
    pub(crate) fn new(cortical_type: CorticalType, cortical_group: CorticalGroupingIndex) -> Self {
        CorticalAreaMetadataKey {
            cortical_type,
            cortical_group,
        }
    }
}



#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct AccessAgentLookupKey {
    pub(crate) cortical_type: CorticalType,
    pub(crate) agent_index: AgentDeviceIndex,
}

impl AccessAgentLookupKey {
    pub(crate) fn new(cortical_type: CorticalType, agent_index: AgentDeviceIndex) -> Self {
        AccessAgentLookupKey{
            cortical_type,
            agent_index,
        }
    }
}

