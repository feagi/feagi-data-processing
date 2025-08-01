//! XYZP neural coordinate system and data structures.
//!
//! This module implements the core XYZP (X, Y, Z, Potential) coordinate system used
//! throughout FEAGI for neural representation. It provides data structures for
//! individual neurons, efficient collections, cortical organization, and conversion
//! between external data formats and neural representations.
//!
//! # XYZP Coordinate System
//!
//! The XYZP system represents neurons using four dimensions:
//! - **X, Y, Z**: 3D spatial coordinates within a cortical area (u32, 0 to 4.3 billion)
//! - **P**: Potential/activation value representing neural activity (f32, typically 0.0 to 1.0)
//!
//! This spatial representation allows FEAGI to:
//! - Maintain topological relationships from biological neural networks
//! - Enable efficient spatial queries and neighbor finding
//! - Support realistic neural connectivity patterns
//! - Process visual and spatial data with natural coordinate mappings
//!
//! # Core Data Structures
//!
//! ## Individual Neuron Representation
//! - **NeuronXYZP**: Single neuron with XYZP coordinates
//! - Lightweight, copyable structure optimized for individual neuron operations
//!
//! ## Efficient Collections
//! - **NeuronXYZPArrays**: Array-based storage for high-performance batch operations
//! - Parallel arrays for X, Y, Z, P values enabling vectorized processing
//! - Memory-efficient layout for large neuron populations of a single cortical area
//!
//! ## Cortical Organization
//! - **CorticalMappedXYZPNeuronData**: Maps cortical areas to neuron collections
//! - Hierarchical organization matching biological brain structure
//! - Support for network serialization and distribution
//!
//! # Neural Encoding/Decoding System
//!
//! The coders module provides conversion between external data and neural representations:
//!
//! ## Encoder Types
//! - **Linear encoders**: Direct value-to-activation mapping
//! - **Normalized encoders**: Range-constrained mappings for bounded data
//! - **Image encoders**: Spatial encoding for visual information
//!
//! ## Decoder Types
//! - **Neural-to-data conversion**: Extract meaningful values from neural activations
//! - **Multi-channel support**: Process multiple cortical channels simultaneously
//! - **Type-safe conversions**: Ensure data type compatibility
//!
//! # Memory Layout and Performance
//!
//! The XYZP system is optimized for:
//! - **Cache efficiency**: Array-of-structures vs structure-of-arrays based on usage
//! - **Vectorization**: SIMD-friendly data layouts where possible
//! - **Network transmission**: Compact serialization formats
//! - **Parallel processing**: Thread-safe operations for concurrent neural simulation
//!
//! # Usage Patterns
//!
//! ## Individual Neuron Operations
//! ```rust
//! use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZP;
//!
//! // Create and manipulate individual neurons
//! let neuron = NeuronXYZP::new(10, 5, 2, 0.8);
//! let (x, y, z, p) = neuron.as_tuple();
//! ```
//!
//! ## Batch Processing
//! ```rust
//! use feagi_core_data_structures_and_processing::neuron_data::xyzp::{NeuronXYZP, NeuronXYZPArrays};
//!
//! // Efficient batch operations
//! let mut arrays = NeuronXYZPArrays::new(1000).unwrap();
//! arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
//! arrays.add_neuron(&NeuronXYZP::new(4, 5, 6, 0.7));
//! ```
//!
//! ## Cortical Organization
//! ```rust
//! use feagi_core_data_structures_and_processing::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZP, NeuronXYZPArrays};
//! use feagi_core_data_structures_and_processing::genomic_structures::CorticalID;
//!
//!
//! // Efficient batch operations
//! let mut arrays = NeuronXYZPArrays::new(1000).unwrap();
//! arrays.add_neuron(&NeuronXYZP::new(1, 2, 3, 0.5));
//! arrays.add_neuron(&NeuronXYZP::new(4, 5, 6, 0.7));
//! 
//! // Organize neurons by cortical areas
//! let mut cortical_data = CorticalMappedXYZPNeuronData::new();
//! let id = CorticalID::from_string("iVcc00".to_string()).unwrap();
//! cortical_data.insert(id, arrays);
//! ```
//!
//! # Integration with FEAGI Systems
//!
//! The XYZP module integrates with:
//! - **I/O Data**: Through neural encoders and decoders
//! - **Genomic Structures**: For cortical area identification and validation
//! - **Stream Processing**: For real-time neural data transformation
//! - **Byte Structures**: For efficient network serialization
//! - **Cortical Types**: For determining appropriate encoding schemes

mod neuron_xyzp;
mod neuron_xyzp_arrays;
mod cortical_mapped_xyzp_neuron_data;
mod coders;

pub use neuron_xyzp::NeuronXYZP;
pub use neuron_xyzp_arrays::NeuronXYZPArrays;
pub use cortical_mapped_xyzp_neuron_data::CorticalMappedXYZPNeuronData;
pub use coders::{NeuronCoderVariantType};

pub(crate) use coders::{NeuronXYZPEncoder, NeuronXYZPDecoder};