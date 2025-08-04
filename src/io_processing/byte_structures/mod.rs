//! FEAGI byte structure serialization and deserialization framework.
//!
//! This module provides a standardized binary format for serializing and deserializing
//! various data structures used in FEAGI processing. The system supports multiple
//! serialization formats and can handle both single structures and multi-structure
//! containers within a unified byte stream.
//!
//! # Architecture
//!
//! The framework consists of three main components:
//!
//! - **FeagiByteStructureType**: Enumeration of supported serialization formats
//! - **FeagiByteStructure**: Core container for serialized byte data with validation
//! - **FeagiByteStructureCompatible**: Trait for types that can be serialized/deserialized
//!
//! # Binary Format
//!
//! All FEAGI byte structures follow a standardized header format:
//! ```text
//! [Type (1 byte)][Version (1 byte)][Format-specific data...]
//! ```
//!
//! Multi-structure containers add additional headers for indexing multiple structures.
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