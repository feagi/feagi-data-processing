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

/// Byte structure serialization and deserialization framework.
/// 
/// This module provides a comprehensive system for converting FEAGI data structures
/// to and from various byte formats.
pub mod byte_structures;

/// Brain output processing modules for motor control data.


/// Centralized error handling for data processing operations.
pub mod error;

pub mod miscellaneous_types;
pub mod genome_definitions;
pub mod io_cache;
pub mod data_types;



pub mod io_data;

pub mod neuron_data;
pub mod genomic_structures;

#[cfg(test)]
mod tests {
    // Tests of each module are in the mod file of each module, and are run from there
}
