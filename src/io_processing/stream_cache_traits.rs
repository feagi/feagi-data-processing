use std::fmt;
use crate::genomic_structures::CorticalID;
use crate::genomic_structures::{CorticalGroupingIndex};

pub trait StreamXYZPCache<T: fmt::Display> : fmt::Display {
    fn get_cached_data(&self) -> &T;
    fn get_channel_status(&self) -> &DeviceChannelStatus;
    fn get_grouping_index(&self) -> &GroupingIndex;
    fn get_channel_index(&self) -> &ChannelIndex;
    fn get_cortical_area_id(&self) -> &CorticalID;
}