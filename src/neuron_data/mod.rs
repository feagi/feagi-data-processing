//! Neuron data structures and processing for FEAGI neural networks.
//!
//! This module provides the core data structures and algorithms for representing,
//! encoding, and decoding neural information in the FEAGI system. It defines how
//! individual neurons and collections of neurons are stored, transmitted, and
//! processed within the neural network.
//!
//! # Core Components
//!
//! ## XYZP Module
//! The `xyzp` module contains the fundamental neural data structures:
//! - **NeuronXYZP**: Individual neuron representation
//! - **NeuronXYZPArrays**: Efficient collections of neurons for processing from within a single cortical area
//! - **CorticalMappedXYZPNeuronData**: Neuron data organized by cortical areas
//! - **Neural Coders**: Conversion between I/O data and neural representations
//!
pub mod xyzp;