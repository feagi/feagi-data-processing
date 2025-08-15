//! Stream cache processors for real-time data transformation in FEAGI.
//!
//! This module provides a framework for processing streaming data through a pipeline
//! of transformations. The system allows chaining multiple processors together to
//! create complex data processing workflows.
//!
//! # Architecture
//!
//! - **StreamCacheProcessor trait**: Defines the interface that all processors must implement
//! - **ProcessorRunner**: Orchestrates execution of a chain of processors, ensuring type compatibility
//! - **processors module**: Contains concrete implementations of various processor types
//!
//! # Usage
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::io_processing::{
//!     StreamCacheProcessor, 
//!     processors::LinearScaleTo0And1Processor
//! };
//!
//! // Create a processor that scales values from [0, 100] to [0, 1]
//! let processor = LinearScaleTo0And1Processor::new(0.0, 100.0, 50.0).unwrap();
//!
//! // Processors can be chained together using ProcessorRunner
//! ```

mod stream_cache_processor_trait;
mod processor_runner;
pub mod processors;
mod verify_stream_cache_processor_chain;

pub use stream_cache_processor_trait::StreamCacheProcessor;
pub(crate) use processor_runner::ProcessorRunner;
pub(crate) use verify_stream_cache_processor_chain::*;

