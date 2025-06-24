use std::time::Instant;


pub use device_group_cache::ChannelIndex as ChannelIndex;
pub use device_group_cache::GroupIndex as GroupIndex;

pub mod input_cache;
pub mod input_workers;
mod output_workers;
mod device_group_cache;

pub trait IOCacheWorker<T> {
    fn get_last_data_update_time(&self) -> Instant;
}