use std::collections::HashMap;
use crate::io_data::IOTypeData;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CallbackSubscriberID {
    pub id: usize
}

impl CallbackSubscriberID {
    pub fn new(id: usize) -> CallbackSubscriberID {
        CallbackSubscriberID { id }
    }
}

pub struct CallBackManager {
    callbacks: HashMap<usize, Box<dyn Fn(&IOTypeData) + Send + Sync>>,
    next_id: usize // This will fail if more than 18,446,744,073,709,551,615 callbacks are registered (on 64bit). Too Bad!
}

impl  CallBackManager {
    pub fn new() -> CallBackManager {
        Self {
            callbacks: HashMap::new(),
            next_id: 0
        }
    }

    pub fn register<F>(&mut self, callback: F) -> CallbackSubscriberID
    where
        F: Fn(&IOTypeData) + Send + Sync + 'static,
    {
        let id = self.next_id;
        self.callbacks.insert(id, Box::new(callback));
        self.next_id += 1;
        CallbackSubscriberID::new(id)
    }

    /// Deregister a callback by ID
    pub fn deregister(&mut self, id: CallbackSubscriberID) -> bool {
        self.callbacks.remove(&id.id).is_some()
    }

    /// Call all registered callbacks with a parameter
    pub(crate) fn emit(&self, value: &IOTypeData) {
        for cb in self.callbacks.values() {
            cb(value);
        }
    }
    
}


