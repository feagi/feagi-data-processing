use std::time::Instant;


pub mod input_cache;
pub mod input_workers;
mod output_workers;

pub trait IOCacheWorker<T> {
    fn get_last_data_update_time(&self) -> Instant;
}