mod float_output_workers;

use std::fmt;
use crate::io_cache::callback_manager::{CallBackManager};
use super::IOCacheWorker;

trait OutputCacheWorker<T: fmt::Display>: IOCacheWorker<T> {
    fn feagi_sent_value(&mut self, new_value: T); // use translator at higher level to get data
    
    fn feagi_sent_nothing(&mut self); // We should be "zeroing"
    
    fn callback_manager(&mut self) -> &mut CallBackManager<T>;
}
