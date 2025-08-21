//! FEAGI genomic structures for cortical area management and neural organization.
//!
//! This module provides the data structures and types used to define, identify,
//! and manage brain structures within FEAGI.
//!
//! # Architecture Overview
//!
//! The genomic structures system consists of several interconnected basic_components:
//!
//! - **CorticalID**: Unique identifiers for cortical areas with format validation
//! - **CorticalType**: Classification system for different types of cortical areas
//! - **Index Types**: Type-safe wrappers for various indexing schemes
//! - **Single Channel Dimensions**: Spatial dimension management for cortical area channels
//! - **Single Channel Dimension Ranges**: Validation constraints for cortical area channels sizing
//!
//! # Cortical Area Organization
//!
//! FEAGI organizes neural processing into discrete cortical areas, each with:
//! - A unique 6-character ASCII identifier (CorticalID)
//! - A specific type classification (Core, Sensory, Motor, Custom, Memory)
//! - Dimensional properties defining its spatial structure
//! - Channel indexing for input/output mapping
//!
//! # Identifier Format
//!
//! Cortical IDs follow a structured 6-character format:
//! ```text
//! [Type Prefix][Content][Index]
//! 
//! Examples:
//! - "ivis00" = Input Vision sensor, index 0
//! - "omot01" = Output Motor, index 1  
//! - "_death" = Core Death area
//! - "cfn3jf" = Custom area
//! - "mfhwf1" = Memory area
//! ```
//!
//! # Type System
//!
//! The system supports five main cortical area types:
//! - **Core**: Universal cortical areas in all genomes (death, power)
//! - **Sensory**: Input processing areas (vision, touch, etc.)
//! - **Motor**: Output control areas (motors, actuators)
//! - **Custom**: User-defined processing areas
//! - **Memory**: Persistent storage areas
//!
//! # Usage Example
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::genomic_structures::*;
//!
//! // Create a vision sensor cortical area
//! let vision_id = CorticalID::new_sensor_cortical_area_id(
//!     SensorCorticalType::ImageCameraCenter,
//!     CorticalGroupingIndex::from(0)
//! ).unwrap();
//!
//! // Get its type information
//! let cortical_type = vision_id.get_cortical_type();
//! let channel_range = cortical_type.try_get_channel_size_boundaries().unwrap();
//! ```

mod index_types;
//mod cortical_area_dimensions;
mod cortical_id;
mod cortical_type;
mod single_channel_dimensions;
mod single_channel_dimension_range;

pub use index_types::*;
// pub use cortical_area_dimensions::CorticalAreaDimensions;
pub use cortical_type::{CorticalType, CoreCorticalType, MotorCorticalType, SensorCorticalType};
pub use single_channel_dimensions::{SingleChannelDimensions};
pub use single_channel_dimension_range::{SingleChannelDimensionRange};
pub use cortical_id::CorticalID;