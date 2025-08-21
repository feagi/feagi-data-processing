//! Input/Output data types and structures for FEAGI processing.
//!
//! This module provides the core data types and structures used for handling input and output
//! data in the FEAGI neural system. It includes strongly-typed wrappers for different kinds
//! of data (floats, images, JSON), validation, conversion utilities, and serialization support.
//!
//! # Core Data Types
//!
//! The module is organized around several key data types:
//!
//! - **IOTypeData**: Unified enum containing all supported data types with values
//! - **IOTypeVariant**: Type identifiers without values for type checking and classification
//! - **ImageFrame**: Single image/frame data with processing capabilities
//! - **SegmentedImageFrame**: Multi-segment vision frames for peripheral vision simulation
//! - **JsonStructure**: JSON data with FEAGI bytes structure compatibility
//!
//! # Type Safety and Validation
//!
//! All data types include validation to ensure:
//! - Numeric values are finite (no NaN or infinite values)
//! - Normalized values stay within their specified ranges
//! - Image data maintains consistent dimensions and formats
//! - JSON data is well-formed and parseable
//!
//! # Data Flow
//!
//! The typical data flow involves:
//! 1. **Input**: Raw data from sensors, files, or network sources
//! 2. **Validation**: Type checking and range validation during construction
//! 3. **Processing**: Stream cache processing transform data between types
//! 4. **Neural encoding**: Conversion to neural representations for FEAGI processing
//! 5. **Output**: Serialization for storage, transmission, or motor control
//!
//! # Usage Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::io_data::*;
//!
//! // Create validated float data
//! let sensor_reading = IOTypeData::new_f32(25.5).unwrap();
//! let normalized = IOTypeData::new_0_1_f32(0.75).unwrap();
//!
//! // Type checking
//! assert!(IOTypeVariant::F32.is_of(&sensor_reading));
//! assert!(IOTypeVariant::F32Normalized0To1.is_of(&normalized));
//! ```
//!
//! # Integration with FEAGI Systems
//!
//! This module integrates with:
//! - **Stream Cache Processors**: For real-time data transformation
//! - **Byte Structures**: For efficient serialization and network transmission
//! - **Genomic Structures**: For cortical area type definitions and constraints
//! - **Neuron Data**: For conversion to neural representations

mod image;
mod io_types;

pub use image::{ImageFrame, SegmentedImageFrame, ImageFrameTransformer, ImageFrameSegmentator};
pub use image::descriptors as image_descriptors;
pub use crate::feagi_json::FeagiJSON;
pub use io_types::{IOTypeData, IOTypeVariant};