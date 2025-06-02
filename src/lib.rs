//! # FEAGI Data Processing Library
//! 
//! This crate provides comprehensive data structures and processing utilities for the FEAGI 
//! (Framework for Evolutionary Artificial General Intelligence) system. It handles the core
//! data processing requirements for neural simulation, including neuron data management,
//! cortical area identification, serialization/deserialization, and brain input/output processing.
//!
//! ## Module Overview
//!
//! - [`byte_structures`]: Serialization and deserialization framework for FEAGI data formats
//! - [`brain_input`]: Processing modules for sensory input data (primarily vision systems)
//! - [`brain_output`]: Processing modules for motor output data
//! - [`error`]: Centralized error handling for all data processing operations
//! - [`neuron_data`]: Core data structures for managing neuron information and properties
//! - [`cortical_data`]: Cortical area identification and management utilities
//!
//! ## Examples
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::neuron_data::NeuronXYCPArrays;
//! use feagi_core_data_structures_and_processing::cortical_data::CorticalID;
//! use std::collections::HashMap;
//!
//! // Create a cortical ID
//! let cortical_id = CorticalID::from_str("iv00CC").unwrap();
//!
//! // Create neuron data structure
//! let mut neuron_data = NeuronXYCPArrays::new(1000).unwrap();
//!
//! // Organize by cortical areas
//! let mut cortical_mapped_data = HashMap::new();
//! cortical_mapped_data.insert(cortical_id, neuron_data);
//! ```

/// Byte structure serialization and deserialization framework.
/// 
/// This module provides a comprehensive system for converting FEAGI data structures
/// to and from various byte formats.
pub mod byte_structures;

/// Brain input processing modules for sensory data.
pub mod brain_input;

/// Brain output processing modules for motor control data.

pub mod brain_output;

/// Centralized error handling for data processing operations.
pub mod error;

/// Core neuron data structures and management utilities.
/// 
/// Provides efficient data structures for storing and manipulating neuron information
/// including positions (X, Y coordinates), channels, and potential values. Supports
/// organization by cortical areas and includes utilities for memory management and
/// vector operations.
pub mod neuron_data;

/// Cortical area identification and management.
pub mod cortical_data;

#[cfg(test)]
mod tests {
    // Tests of each module are in the mod file of each module, and are run from there
}
