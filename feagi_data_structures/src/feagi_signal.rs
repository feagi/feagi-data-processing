use std::collections::HashMap;
use std::fmt::Debug;
use crate::{define_index, FeagiDataError};

define_index!(FeagiSignalIndex, u32, "A unique identifier for a subscription to a FeagiSignal");

pub struct FeagiSignal<'a, T> { // Totally not stolen concept from Godot
    listeners: HashMap<FeagiSignalIndex, Box<dyn Fn(&T) + Send + Sync + 'a>>,
    next_index: u32,
}


impl<'a, T> FeagiSignal<'a, T> {
    pub fn new() -> Self {
        Self { listeners: HashMap::new(), next_index: 0 }
    }

    pub fn connect<F>(&mut self, f: F) -> FeagiSignalIndex // Will overflow after 4 billion subscriptions. Too bad!
    where
        F: Fn(&T) + Send + Sync + 'a,  // Changed from 'static to 'a
    {
        self.listeners.insert(self.next_index.into(), Box::new(f));
        self.next_index += 1;
        (self.next_index - 1).into()
    }

    pub fn disconnect(&mut self, index: FeagiSignalIndex) -> Result<(), FeagiDataError> {
        if self.listeners.remove(&index).is_some() {
            return Ok(())
        }
        Err(FeagiDataError::BadParameters(format!("No subscription found with identifier {}!", index)))
    }

    pub fn emit(&self, value: T) {
        for f in &self.listeners {
            f.1(&value);
        }
    }
    
    /// Emit with additional context parameters that weren't stored in the closure
    /// This allows passing runtime parameters without capturing them
    pub fn emit_with_context<C>(&self, value: T, context: C) 
    where
        C: Clone
    {
        for f in &self.listeners {
            f.1(&value);
            // Note: Standard FeagiSignal can't use context, see FeagiSignalWithContext instead
        }
    }
}

/// A signal that passes context when emitting, allowing callbacks to receive runtime parameters
pub struct FeagiSignalWithContext<T, C> {
    listeners: HashMap<FeagiSignalIndex, Box<dyn Fn(&T, &C) + Send + Sync>>,
    next_index: u32,
}

impl<T, C> FeagiSignalWithContext<T, C> {
    pub fn new() -> Self {
        Self { listeners: HashMap::new(), next_index: 0 }
    }

    pub fn connect<F>(&mut self, f: F) -> FeagiSignalIndex
    where
        F: Fn(&T, &C) + Send + Sync + 'static,
    {
        self.listeners.insert(self.next_index.into(), Box::new(f));
        self.next_index += 1;
        (self.next_index - 1).into()
    }

    pub fn disconnect(&mut self, index: FeagiSignalIndex) -> Result<(), FeagiDataError> {
        if self.listeners.remove(&index).is_some() {
            return Ok(())
        }
        Err(FeagiDataError::BadParameters(format!("No subscription found with identifier {}!", index)))
    }

    pub fn emit_with_context(&self, value: &T, context: &C) {
        for f in &self.listeners {
            f.1(value, context);
        }
    }
}

impl<T, C> Debug for FeagiSignalWithContext<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeagiSignalWithContext")
            .field("listener_count", &self.listeners.len())
            .field("next_index", &self.next_index)
            .field("listener_indices", &self.listeners.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl<'a, T> Debug for FeagiSignal<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeagiSignal")
            .field("listener_count", &self.listeners.len())
            .field("next_index", &self.next_index)
            .field("listener_indices", &self.listeners.keys().collect::<Vec<_>>())
            .finish()
    }
}