//! Brain input processing modules for sensory data in FEAGI.
//!
//! This module contains specialized data structures and processing functions for handling
//! sensory input to the FEAGI brain simulation. It focuses on converting external sensory
//! data into formats suitable for neural processing within the FEAGI framework.

/// Vision processing module for image-based sensory input.
/// 
/// Provides comprehensive image processing capabilities for FEAGI, including:
/// - Image frame data structures with uncompressed pixel storage using ndarrays
/// - Segmented vision processing for region-based visual analysis
/// - Visual feature descriptors for pattern recognition
/// - Conversion utilities from pixel data to neural activation patterns
/// 
/// The vision module is optimized for real-time processing and integration
/// with FEAGI's neural simulation pipeline.
pub mod vision;
