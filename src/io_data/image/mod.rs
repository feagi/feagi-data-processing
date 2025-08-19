//! Image processing and vision data structures for FEAGI.
//!
//! This module provides comprehensive image handling capabilities for FEAGI's vision
//! processing systems. It includes structures for single images, segmented vision
//! frames for peripheral vision simulation, and supporting descriptors for image
//! processing configuration.
//!
//! # Core Types
//!
//! - **ImageFrame**: Single image/frame data with processing capabilities
//! - **SegmentedImageFrame**: Multi-segment vision frames dividing images into 9 regions
//! - **descriptors**: Module of umage processing parameters, color spaces, and configuration
//!
//! # Vision Processing Features
//!
//! ## Single Image Processing
//! - Multiple color spaces (Linear, Gamma)
//! - Various channel layouts (Grayscale, RGB, RGBA)
//! - Image operations (cropping, resizing, brightness/contrast)
//! - Neural data conversion for FEAGI processing
//!
//! ## Peripheral Vision Simulation
//! - 3x3 segmented vision with center focus and peripheral regions
//! - Different resolutions per segment to simulate human vision
//! - Configurable processing parameters per segment

mod image_frame;
mod segmented_image_frame;

pub mod descriptors;
mod image_frame_transformer;
mod image_frame_segmentator;

pub use segmented_image_frame::SegmentedImageFrame;
pub use image_frame::ImageFrame;
pub use image_frame_transformer::ImageFrameTransformer;
pub use image_frame_segmentator::ImageFrameSegmentator;
