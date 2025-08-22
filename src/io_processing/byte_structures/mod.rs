//! FEAGI bytes structure serialization and deserialization framework.
//!
//! This module provides a standardized binary format for serializing and deserializing
//! various data structures used in FEAGI processing. The system supports multiple
//! serialization formats and can handle both single structures and multi-structure
//! wrapped_io_data within a unified bytes stream.
//!
//! # Architecture
//!
//! The framework consists of three main basic_components:
//!
//! - **FeagiByteStructureType**: Enumeration of supported serialization formats
//! - **FeagiByteStructure**: Core container for serialized bytes data with validation
//! - **FeagiByteStructureCompatible**: Trait for types that can be serialized/deserialized
//!
//! # Binary Format
//!
//! All FEAGI bytes structures follow a standardized header format:
//! ```text
//! [Type (1 bytes)][Version (1 bytes)][Format-specific data...]
//! ```
//!
//! Multi-structure wrapped_io_data add additional headers for indexing multiple structures.
//!
//! # Supported Formats
//!
//! - **JSON**: Human-readable text serialization
//! - **NeuronCategoricalXYZP**: Optimized binary format for neuron spatial data  
//! - **MultiStructHolder**: Container format for multiple structures
//! 
mod feagi_byte_structure_type;
mod feagi_byte_structure;
mod feagi_byte_structure_compatible;

pub use feagi_byte_structure_type::FeagiByteStructureType;
pub use feagi_byte_structure_compatible::FeagiByteStructureCompatible;
pub use feagi_byte_structure::FeagiByteStructure;