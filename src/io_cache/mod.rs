use std::fmt;
use crate::genome_definitions::identifiers::CorticalID;

pub mod input_workers;
pub mod output_workers;

pub use helpers::GroupingIndex as GroupingIndex;
pub use helpers::ChannelIndex as ChannelIndex;
pub use helpers::ChannelStatus as ChannelStatus;
pub use callback_manager::CallBackManager as CallbackManager;
use crate::genome_definitions::identifiers::InputCorticalType::VisionCenterGray;

mod callback_manager;
mod device_group_cache;
mod helpers;

pub trait IOCacheWorker<T: fmt::Display> : fmt::Display {
    fn get_cached_data(&self) -> T;
    fn get_channel_status(&self) -> ChannelStatus;
    fn get_grouping_index(&self) -> GroupingIndex;
    fn get_channel_index(&self) -> ChannelIndex;
    fn get_cortical_area_id(&self) -> &CorticalID;
}

