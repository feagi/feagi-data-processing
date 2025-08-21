//! Core traits for neural encoding and decoding operations.
//!
//! This module defines the fundamental traits that all neural encoders and decoders
//! must implement. These traits provide a unified interface for converting between
//! external I/O data types and internal neural representations using the XYZP
//! coordinate system.

use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::StreamCacheProcessor;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData};

/// Trait for encoding external data into neural XYZP representations.
///
/// `NeuronXYZPEncoder` defines the interface for converting I/O data types into
/// neural activation patterns within cortical areas. Encoders transform external
/// sensor data into spatial patterns of neural activation that can be processed
/// by the FEAGI neural network.
///
/// # Encoding Process
///
/// 1. **Data Reception**: Accept typed I/O data from external sources
/// 2. **Spatial Mapping**: Convert data values to neural coordinates (X, Y, Z)
/// 3. **Activation Encoding**: Generate appropriate potential (P) values
/// 4. **Neural Writing**: Store activations in cortical mapped neuron data
///
/// # Implementation Requirements
///
/// Implementors must provide:
/// - **Type Declaration**: Specify which I/O data type this encoder handles
/// - **Single Channel Encoding**: Convert data for one cortical channel
/// - **Multi Channel Support**: Efficiently handle multiple channels (optional override)
///
/// # Thread Safety
/// All encoder implementations must be `Sync + Send` for use in multi-threaded
/// neural simulation environments.
pub(crate) trait NeuronXYZPEncoder {
    /// Returns the I/O data type this encoder can process.
    ///
    /// This method specifies which [`IOTypeVariant`] this encoder is designed
    /// to handle. The type system uses this for validation and automatic
    /// encoder selection based on data type.
    ///
    /// # Returns
    /// The [`IOTypeVariant`] that this encoder can process
    fn get_encodable_data_type(&self) -> IOTypeVariant;

    /// Encodes I/O data into neural activations for a single cortical channel.
    ///
    /// This is the core encoding method that converts external data into neural
    /// activation patterns within a specific cortical channel. The implementation
    /// should handle the data type specified by `get_encodable_data_type()`.
    ///
    /// # Arguments
    /// * `wrapped_value` - The I/O data to encode (must match encoder's data type)
    /// * `cortical_channel` - Target channel index within the cortical area
    /// * `write_target` - Cortical neuron data collection to write activations to
    ///
    /// # Returns
    /// * `Ok(())` - Encoding completed successfully
    /// * `Err(FeagiDataProcessingError)` - Encoding failed due to data type mismatch or other error
    ///
    /// # Implementation Notes
    /// - Validate that the input data matches the expected type
    /// - Convert data values to appropriate neural coordinates (X, Y, Z)
    /// - Generate suitable potential (P) values for neural activation
    /// - Write results to the target cortical area using the specified channel
    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError>;
    
    /// Encodes multiple channels of data in a single operation.
    ///
    /// This method processes multiple cortical channels simultaneously, which can
    /// be more efficient than individual single-channel calls. The default
    /// implementation calls `write_neuron_data_single_channel` for each channel,
    /// but can be overridden for better performance.
    ///
    /// # Arguments
    /// * `channels_and_values` - Map of channel indices to their corresponding data
    /// * `write_target` - Cortical neuron data collection to write activations to
    ///
    /// # Returns
    /// * `Ok(())` - All channels encoded successfully
    /// * `Err(FeagiDataProcessingError)` - Encoding failed for at least one channel
    ///
    /// # Performance Notes
    /// Consider overriding this method for:
    /// - Vectorized operations when processing many channels
    /// - Batch optimizations specific to the encoding algorithm
    /// - Shared computation that can be amortized across channels
    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, &IOTypeData>, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, channel, write_target)?;
        };
        Ok(())
    }
}

/// Trait for decoding neural XYZP data back into external I/O formats.
///
/// `NeuronXYZPDecoder` defines the interface for converting neural activation
/// patterns back into meaningful external data types. Decoders extract information
/// from spatial patterns of neural activity and produce typed I/O data that can
/// be used for motor control, data output, or further processing.
///
/// # Decoding Process
///
/// 1. **Neural Reading**: Access neural activation patterns from cortical areas
/// 2. **Pattern Analysis**: Analyze spatial distribution of activations
/// 3. **Value Extraction**: Convert activation patterns to meaningful values
/// 4. **Type Conversion**: Produce appropriate I/O data type
///
/// # Implementation Requirements
///
/// Implementors must provide:
/// - **Type Declaration**: Specify which I/O data type this decoder produces
/// - **Channel Dimensions**: Define spatial layout for reading neural data
/// - **Cortical Sources**: List cortical areas this decoder reads from
/// - **Single Channel Decoding**: Extract data from one cortical channel
/// - **Multi Channel Support**: Efficiently handle multiple channels (optional override)
///
/// # Thread Safety
/// All decoder implementations must be thread-safe for use in multi-threaded
/// neural simulation environments.
pub trait NeuronXYZPDecoder {
    /// Returns the I/O data type this decoder produces.
    ///
    /// This method specifies which [`IOTypeVariant`] this decoder will
    /// generate from neuron data. The type system uses this for validation
    /// and compatibility checking with downstream processing basic_components.
    ///
    /// # Returns
    /// The [`IOTypeVariant`] that this decoder produces
    fn get_decoded_data_type(&self) -> IOTypeVariant;

    /// Returns the spatial channel dimensions this decoder expects.
    ///
    /// This method provides the spatial layout information that defines how
    /// this decoder interprets neural activation patterns. The dimensions
    /// specify the X, Y, and Z extents of the neural space that will be
    /// read during decoding operations.
    ///
    /// # Returns
    /// Reference to the [`SingleChannelDimensions`] defining the spatial layout
    fn get_channel_dimensions(&self) -> &SingleChannelDimensions;

    /// Returns the cortical areas this decoder reads from.
    ///
    /// This method specifies which cortical areas contain the neural data
    /// that this decoder will process. Multiple cortical areas can be
    /// specified for decoders that aggregate information from multiple
    /// brain regions.
    ///
    /// # Returns
    /// Slice of [`CorticalID`] values identifying source cortical areas
    fn get_cortical_id_read_destinations(&self) -> &[CorticalID];
    
    /// Decodes neural activations from a single cortical channel.
    ///
    /// This is the core decoding method that converts neural activation
    /// patterns from a specific cortical channel back into typed I/O data.
    /// The implementation should analyze the spatial pattern of neural
    /// activations and extract meaningful values.
    ///
    /// # Arguments
    /// * `cortical_channel` - Source channel index within the cortical area
    /// * `read_from` - Cortical neuron data collection to read activations from
    ///
    /// # Returns
    /// * `Ok(IOTypeData)` - Successfully decoded data of the appropriate type
    /// * `Err(FeagiDataProcessingError)` - Decoding failed due to insufficient data or other error
    ///
    /// # Implementation Notes
    /// - Read neural activations from the specified cortical channel
    /// - Analyze spatial patterns according to the decoding algorithm
    /// - Convert activation patterns to appropriate data values
    /// - Return data in the format specified by `get_decoded_data_type()`
    fn read_neuron_data_single_channel(&self, cortical_channel: CorticalIOChannelIndex, read_from: &CorticalMappedXYZPNeuronData) -> Result<IOTypeData, FeagiDataProcessingError>;

    /// Decodes neuron activations from multiple channels into I/O data.
    ///
    /// This method processes multiple channels of neuron data in a single call.
    /// The default implementation calls [`read_neuron_data_single_channel`]
    /// for each channel, but can be overridden for better performance.
    ///
    /// # Arguments
    /// * `channels` - List of channel indices to decode
    /// * `read_from` - Neuron data collection to read activations from
    ///
    /// # Returns
    /// * `Ok(Vec<IOTypeData>)` - Vector of decoded data (one per channel)
    /// * `Err(FeagiDataProcessingError)` - Decoding failed for at least one channel
    ///
    /// # Performance Notes
    ///
    /// Consider overriding this method for vectorized processing when dealing
    /// with large numbers of channels or when batch operations are more efficient.
    /// Examples include:
    /// - Shared computation that can be amortized across channels
    /// - Vectorized analysis of activation patterns
    /// - Batch processing optimizations specific to the decoding algorithm
    ///
    /// [`read_neuron_data_single_channel`]: NeuronXYZPDecoder::read_neuron_data_single_channel
    fn read_neuron_data_multi_channel(&self, channels: &[CorticalIOChannelIndex],  read_from: &CorticalMappedXYZPNeuronData) -> Result<Vec<IOTypeData>, FeagiDataProcessingError> {
        let mut output: Vec<IOTypeData> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(*channel, read_from)?);
        };
        Ok(output)
    }
}