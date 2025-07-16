//! Traits for encoding and decoding neuron data between formats.
//!
//! This module defines the core traits used for converting between I/O data formats
//! (sensor inputs, motor outputs) and neuron representations in the FEAGI system.
//! The encoding/decoding system allows different types of data to be mapped to and
//! from the XYZP neuron coordinate system.
//!
//! # Overview
//!
//! The trait system provides two main capabilities:
//! - **Encoding**: Converting I/O data (images, audio, motor commands) into neuron activations
//! - **Decoding**: Converting neuron activations back into I/O data formats
//!
//! # Architecture
//!
//! The system supports both single-channel and multi-channel operations:
//! - **Single-channel**: Process one channel of data at a time
//! - **Multi-channel**: Process multiple channels efficiently in batch
//!
//! # Usage Patterns
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZPEncoder;
//! use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};
//!
//! // Implementing an encoder for a specific data type
//! struct MyCustomEncoder;
//!
//! impl NeuronXYZPEncoder for MyCustomEncoder {
//!     fn get_encoded_data_type(&self) -> IOTypeVariant {
//!         IOTypeVariant::FloatGreyscale
//!     }
//!
//!     fn write_neuron_data_single_channel(
//!         &self,
//!         wrapped_value: &IOTypeData,
//!         cortical_channel: CorticalIOChannelIndex,
//!         cortical_id_targets: &[CorticalID],
//!         write_target: &mut CorticalMappedXYZPNeuronData
//!     ) -> Result<(), FeagiDataProcessingError> {
//!         // Implementation details...
//!         Ok(())
//!     }
//! }
//! ```
//!
//! # Performance Considerations
//!
//! - Multi-channel operations are generally more efficient than repeated single-channel calls
//! - Encoders should be stateless for thread safety and reusability
//! - Consider vectorization opportunities for batch processing
//!
//! # Future Improvements
//!
//! - Potential enum-based implementation using `enum_dispatch` for performance
//! - Vectorized multi-channel operations to reduce iteration overhead
//! - Specialized encoders for different cortical area types

use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex};
use crate::io_data::{IOTypeData, IOTypeVariant};
use super::{CorticalMappedXYZPNeuronData};

/// Trait for encoding I/O data into neuron activations.
///
/// This trait defines the interface for converting various types of input/output data
/// (such as sensor readings, images, audio) into neuron activations within the FEAGI
/// system. Implementors specify how their data type maps to the XYZP coordinate system.
///
/// # Design Principles
///
/// - **Stateless**: Encoders should not maintain state between calls
/// - **Type-specific**: Each encoder handles one specific [`IOTypeVariant`]
/// - **Channel-aware**: Supports both single and multi-channel operations
/// - **Cortical mapping**: Can write to multiple cortical areas simultaneously
///
/// # Implementation Notes
///
/// Implementors must provide single-channel encoding logic. Multi-channel encoding
/// has a default implementation that calls the single-channel method in a loop,
/// but this can be overridden for performance optimization.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::neuron_data::xyzp::coder_traits::NeuronXYZPEncoder;
/// use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};
/// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalID, CorticalIOChannelIndex};
/// use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
/// use feagi_core_data_structures_and_processing::error::FeagiDataProcessingError;
///
/// struct GrayscaleImageEncoder;
///
/// impl NeuronXYZPEncoder for GrayscaleImageEncoder {
///     fn get_encoded_data_type(&self) -> IOTypeVariant {
///         IOTypeVariant::FloatGreyscale
///     }
///
///     fn write_neuron_data_single_channel(
///         &self,
///         wrapped_value: &IOTypeData,
///         cortical_channel: CorticalIOChannelIndex,
///         cortical_id_targets: &[CorticalID],
///         write_target: &mut CorticalMappedXYZPNeuronData
///     ) -> Result<(), FeagiDataProcessingError> {
///         // Convert grayscale image data to neuron activations
///         // Implementation would map pixel intensities to neuron potentials
///         Ok(())
///     }
/// }
/// ```
pub trait NeuronXYZPEncoder {
    /// Returns the I/O data type this encoder handles.
    ///
    /// This method specifies which [`IOTypeVariant`] this encoder is designed
    /// to process. The type system uses this for validation and routing.
    ///
    /// # Returns
    ///
    /// The [`IOTypeVariant`] that this encoder can process.
    fn get_encoded_data_type(&self) -> IOTypeVariant;

    /// Encodes single-channel I/O data into neuron activations.
    ///
    /// This is the core encoding method that converts one channel of I/O data
    /// into neuron activations within the specified cortical areas.
    ///
    /// # Arguments
    ///
    /// * `wrapped_value` - The I/O data to encode
    /// * `cortical_channel` - Channel index within the cortical area
    /// * `cortical_id_targets` - List of cortical areas to write to
    /// * `write_target` - Neuron data collection to write activations to
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a [`FeagiDataProcessingError`] on failure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use feagi_core_data_structures_and_processing::neuron_data::xyzp::coder_traits::NeuronXYZPEncoder;
    /// # use feagi_core_data_structures_and_processing::io_data::IOTypeData;
    /// # use feagi_core_data_structures_and_processing::genomic_structures::{CorticalID, CorticalIOChannelIndex};
    /// # use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
    /// # struct MyEncoder;
    /// # impl NeuronXYZPEncoder for MyEncoder {
    /// #     fn get_encoded_data_type(&self) -> feagi_core_data_structures_and_processing::io_data::IOTypeVariant { todo!() }
    /// #     fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, cortical_id_targets: &[CorticalID], write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), feagi_core_data_structures_and_processing::error::FeagiDataProcessingError> { todo!() }
    /// # }
    /// # let encoder = MyEncoder;
    /// # let io_data = todo!(); // IOTypeData instance
    /// # let channel = CorticalIOChannelIndex::new(0).unwrap();
    /// # let cortical_areas = vec![]; // Vec<CorticalID>
    /// # let mut neuron_data = CorticalMappedXYZPNeuronData::new();
    ///
    /// // Encode data for channel 0 into specified cortical areas
    /// encoder.write_neuron_data_single_channel(
    ///     &io_data,
    ///     channel,
    ///     &cortical_areas,
    ///     &mut neuron_data
    /// )?;
    /// # Ok::<(), feagi_core_data_structures_and_processing::error::FeagiDataProcessingError>(())
    /// ```
    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, cortical_id_targets: &[CorticalID], write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError>;

    /// Encodes multi-channel I/O data into neuron activations.
    ///
    /// This method processes multiple channels of data in a single call.
    /// The default implementation calls [`write_neuron_data_single_channel`]
    /// for each channel, but can be overridden for better performance.
    ///
    /// # Arguments
    ///
    /// * `channels_and_values` - Map of channel indices to their data
    /// * `cortical_id_targets` - List of cortical areas to write to
    /// * `write_target` - Neuron data collection to write activations to
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a [`FeagiDataProcessingError`] on failure.
    ///
    /// # Performance Notes
    ///
    /// Consider overriding this method for vectorized processing when dealing
    /// with large numbers of channels or when batch operations are more efficient.
    ///
    /// [`write_neuron_data_single_channel`]: NeuronXYZPEncoder::write_neuron_data_single_channel
    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, &IOTypeData>, cortical_id_targets: &[CorticalID], write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, channel, cortical_id_targets, write_target)?;
        };
        Ok(())
    }
}

/// Trait for decoding neuron activations into I/O data.
///
/// This trait defines the interface for converting neuron activations back into
/// various types of output data (such as motor commands, display data, audio).
/// Implementors specify how neuron activations in the XYZP coordinate system
/// map back to their specific data type.
///
/// # Design Principles
///
/// - **Stateless**: Decoders should not maintain state between calls
/// - **Type-specific**: Each decoder produces one specific [`IOTypeVariant`]
/// - **Channel-aware**: Supports both single and multi-channel operations
/// - **Read-only**: Decoders only read from neuron data, never modify it
///
/// # Implementation Notes
///
/// Implementors must provide single-channel decoding logic. Multi-channel decoding
/// has a default implementation that calls the single-channel method in a loop,
/// but this can be overridden for performance optimization.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::neuron_data::xyzp::coder_traits::NeuronXYZPDecoder;
/// use feagi_core_data_structures_and_processing::io_data::{IOTypeData, IOTypeVariant};
/// use feagi_core_data_structures_and_processing::genomic_structures::CorticalIOChannelIndex;
/// use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
/// use feagi_core_data_structures_and_processing::error::FeagiDataProcessingError;
///
/// struct MotorCommandDecoder;
///
/// impl NeuronXYZPDecoder for MotorCommandDecoder {
///     fn get_decoded_data_type(&self) -> IOTypeVariant {
///         IOTypeVariant::FloatServoPosition
///     }
///
///     fn read_neuron_data_single_channel(
///         &self,
///         cortical_channel: CorticalIOChannelIndex,
///         neuron_data: &CorticalMappedXYZPNeuronData
///     ) -> Result<IOTypeData, FeagiDataProcessingError> {
///         // Convert neuron activations to motor command data
///         // Implementation would aggregate neuron potentials into motor signals
///         todo!() // Return appropriate IOTypeData
///     }
/// }
/// ```
pub trait NeuronXYZPDecoder {
    /// Returns the I/O data type this decoder produces.
    ///
    /// This method specifies which [`IOTypeVariant`] this decoder will
    /// generate from neuron data. The type system uses this for validation.
    ///
    /// # Returns
    ///
    /// The [`IOTypeVariant`] that this decoder produces.
    fn get_decoded_data_type(&self) -> IOTypeVariant;

    /// Decodes neuron activations from a single channel into I/O data.
    ///
    /// This is the core decoding method that converts neuron activations
    /// within a specific channel back into I/O data format.
    ///
    /// # Arguments
    ///
    /// * `cortical_channel` - Channel index within the cortical area to read from
    /// * `neuron_data` - Neuron data collection to read activations from
    ///
    /// # Returns
    ///
    /// The decoded [`IOTypeData`] on success, or a [`FeagiDataProcessingError`] on failure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use feagi_core_data_structures_and_processing::neuron_data::xyzp::coder_traits::NeuronXYZPDecoder;
    /// # use feagi_core_data_structures_and_processing::genomic_structures::CorticalIOChannelIndex;
    /// # use feagi_core_data_structures_and_processing::neuron_data::xyzp::CorticalMappedXYZPNeuronData;
    /// # struct MyDecoder;
    /// # impl NeuronXYZPDecoder for MyDecoder {
    /// #     fn get_decoded_data_type(&self) -> feagi_core_data_structures_and_processing::io_data::IOTypeVariant { todo!() }
    /// #     fn read_neuron_data_single_channel(&self, cortical_channel: CorticalIOChannelIndex, neuron_data: &CorticalMappedXYZPNeuronData) -> Result<feagi_core_data_structures_and_processing::io_data::IOTypeData, feagi_core_data_structures_and_processing::error::FeagiDataProcessingError> { todo!() }
    /// # }
    /// # let decoder = MyDecoder;
    /// # let channel = CorticalIOChannelIndex::new(0).unwrap();
    /// # let neuron_data = CorticalMappedXYZPNeuronData::new();
    ///
    /// // Decode data from channel 0
    /// let decoded_data = decoder.read_neuron_data_single_channel(
    ///     channel,
    ///     &neuron_data
    /// )?;
    /// # Ok::<(), feagi_core_data_structures_and_processing::error::FeagiDataProcessingError>(())
    /// ```
    fn read_neuron_data_single_channel(&self, cortical_channel: CorticalIOChannelIndex, neuron_data: &CorticalMappedXYZPNeuronData) -> Result<IOTypeData, FeagiDataProcessingError>;

    /// Decodes neuron activations from multiple channels into I/O data.
    ///
    /// This method processes multiple channels of neuron data in a single call.
    /// The default implementation calls [`read_neuron_data_single_channel`]
    /// for each channel, but can be overridden for better performance.
    ///
    /// # Arguments
    ///
    /// * `neuron_data` - Neuron data collection to read activations from
    /// * `channels` - List of channel indices to decode
    ///
    /// # Returns
    ///
    /// A vector of decoded [`IOTypeData`] (one per channel) on success,
    /// or a [`FeagiDataProcessingError`] on failure.
    ///
    /// # Performance Notes
    ///
    /// Consider overriding this method for vectorized processing when dealing
    /// with large numbers of channels or when batch operations are more efficient.
    ///
    /// [`read_neuron_data_single_channel`]: NeuronXYZPDecoder::read_neuron_data_single_channel
    fn read_neuron_data_multi_channel(&self, neuron_data: &CorticalMappedXYZPNeuronData, channels: &[CorticalIOChannelIndex]) -> Result<Vec<IOTypeData>, FeagiDataProcessingError> {
        let mut output: Vec<IOTypeData> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(*channel, neuron_data)?);
        };
        Ok(output)
    }
}