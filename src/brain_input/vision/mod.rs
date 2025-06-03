//! Vision processing module for image-based sensory input in FEAGI.
//!
//! This module provides comprehensive image processing capabilities for the FEAGI brain
//! simulation system. It handles conversion of visual data into neural activation patterns
//! suitable for processing by artificial neural networks.
//!
//! ## Core Components
//!
//! ### Image Frame Processing ([`image_frame`])
//! - Uncompressed pixel data storage using efficient ndarray structures
//! - Multi-channel image support (RGB, RGBA, grayscale)
//! - Memory-efficient operations for real-time processing
//! - Integration with external image sources and cameras
//!
//! ### Segmented Vision ([`segmented_vision_frame`])
//! - Region-based image analysis and processing
//! - Semantic segmentation support for object detection
//! - Efficient processing of image regions for focused analysis
//! - Support for multiple segmentation algorithms
//!
//! ### Visual Descriptors ([`descriptors`])
//! - Helper structures used to describe operations to be done on images
//!
//!
//!
//! ## Examples
//!
//! ```rust
//! use ndarray::Array3;
//! use feagi_core_data_structures_and_processing::brain_input::vision::*;
//! use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
//!
//! // Process an RGB image frame
//! let image_data: Array3<u8> = Array3::zeros((480, 640, 3)); // Height x Width x Channels
//! // ... populate image_data from camera or file
//!
//! // Convert to neuron activation patterns
//! let mut neuron_data = NeuronXYZPArrays::new(480 * 640 * 3).unwrap();
//! // ... apply processing pipeline
//! ```

use ndarray::{s, Array3, ArrayView3};
use crate::error::DataProcessingError;
use crate::neuron_data::neuron_arrays::NeuronXYZPArrays;
use descriptors::*;

/// Segmented vision processing for region-based image analysis.
/// 
/// Provides functionality for processing images into a central segment and peripheral
/// vision segments.
pub mod segmented_vision_frame;

/// Visual feature descriptors and extraction helpers.
pub mod descriptors;

/// Core image frame data structure and processing functions.
pub mod image_frame;

//pub mod quick_image_diff; // TODO


