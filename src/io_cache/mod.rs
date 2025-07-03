use std::fmt;
use crate::genome_definitions::identifiers::CorticalID;

pub mod input_workers;
pub mod output_workers;

pub use indexers::GroupingIndex as GroupingIndex;
pub use indexers::ChannelIndex as ChannelIndex;
pub use indexers::DeviceChannelStatus as ChannelStatus;
pub use callback_manager::CallBackManager as CallbackManager;
use crate::genome_definitions::identifiers::InputCorticalType::VisionCenterGray;

mod callback_manager;
mod device_group_cache;
mod indexers;

pub trait IOCacheWorker<T: fmt::Display> : fmt::Display {
    fn get_cached_data(&self) -> &T;
    fn get_channel_status(&self) -> &DeviceChannelStatus;
    fn get_grouping_index(&self) -> &GroupingIndex;
    fn get_channel_index(&self) -> &ChannelIndex;
    fn get_cortical_area_id(&self) -> &CorticalID;
}

