//! Stream cache processing for real-time data transformation in FEAGI.
//!
//! This module provides a framework for processing streaming data through a pipeline
//! of transformations. The system allows chaining multiple processing together to
//! create complex data processing workflows.
//!
//! # Architecture
//!
//! - **StreamCacheProcessor trait**: Defines the interface that all processing must implement
//! - **ProcessorRunner**: Orchestrates execution of a chain of processing, ensuring type compatibility
//! - **processing module**: Contains concrete implementations of various processor types


mod stream_cache_processor_trait;
mod processor_runner;
pub mod processors;
mod verify_stream_cache_processor_chain;

pub use stream_cache_processor_trait::StreamCacheProcessor;
pub(crate) use processor_runner::ProcessorRunner;
pub(crate) use verify_stream_cache_processor_chain::*;

