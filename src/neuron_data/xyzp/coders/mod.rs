//! Neural encoding and decoding system for XYZP data conversion.
//!
//! This module provides the encoding and decoding infrastructure for converting between
//! external I/O data types and internal neural representations using the XYZP coordinate
//! system. It implements various coding schemes optimized for different types of data
//! and neural processing requirements.
//!
//! # Neural Coding Overview
//!
//! Neural coding in FEAGI converts external data into spatial patterns of neural activation
//! within cortical areas. Different data types require different encoding strategies:
//!
//! ## Supported Data Types
//! - **Continuous values**: Linear and normalized float encoders
//! - **Bidirectional signals**: Positive/negative value encoding
//! - **Visual data**: Spatial image frame encoding
//! - **Complex signals**: Multi-channel and segmented data
//!
//! # Encoder Types
//!
//! ## Linear Encoders
//! - **F32Normalized0To1_Linear**: Direct mapping of [0,1] values to neural activations
//! - Preserves value relationships with minimal transformation
//! - Suitable for probability, brightness, or other naturally normalized data
//!
//! ## Bidirectional Encoders
//! - **F32NormalizedM1To1_PSPBidirectional**: Post-synaptic potential encoding
//! - **F32NormalizedM1To1_SplitSignDivided**: Separate positive/negative neural populations
//! - Handle bidirectional control signals (motors, steering, etc.)
//!
//! ## Visual Encoders
//! - **ImageFrame**: Spatial encoding of 2D image data to neural coordinates
//! - **SegmentedImageFrame**: Multi-region encoding for peripheral vision simulation
//! - Maintains spatial relationships in neural representation
//!
//! # Decoder Types
//!
//! Decoders reverse the encoding process, extracting meaningful values from neural
//! activation patterns. They support both single-channel and multi-channel operations
//! for efficient batch processing.
//!
//! # Architecture
//!
//! ## Trait System
//! - **NeuronXYZPEncoder**: Trait for data-to-neural conversion
//! - **NeuronXYZPDecoder**: Trait for neural-to-data conversion
//! - Type-safe interfaces ensuring encoder/decoder compatibility
//!
//! ## Type Variants
//! - **NeuronCoderVariantType**: Enum identifying available coding schemes
//! - Factory methods for creating appropriate encoder/decoder instances
//! - Cortical area type integration for automatic coder selection
//!
//! # Usage Patterns
//!
//!
//! # Performance Considerations
//!
//! - **Batch operations**: Multi-channel methods for processing multiple channels efficiently
//! - **Memory locality**: Encoders optimized for spatial data access patterns
//! - **Type specialization**: Different encoders optimized for specific data characteristics
//! - **Vectorization**: Array-based operations where possible
//!
//! # Integration Points
//!
//! The coding system integrates with:
//! - **Cortical Types**: Automatic coder selection based on cortical area type
//! - **Stream Processing**: Real-time encoding/decoding in data pipelines
//! - **I/O Data Types**: Type-safe conversion between external and internal formats
//! - **Genomic Structures**: Channel dimension validation and constraints

mod coder_traits;
mod coder_types;
mod decoders;
mod encoders;

// The only thing that *may* be used outside of this crate is the NeuronEncoderVariantType enum.
// The encoder logic is internal to this crate and nothing needs to know the details of the
// encoders themselves, just their trait (which is spawned by the methods of NeuronEncoderVariantType)

pub use coder_types::{NeuronCoderVariantType};
pub(crate) use coder_traits::{NeuronXYZPEncoder, NeuronXYZPDecoder};