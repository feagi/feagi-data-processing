use std::collections::HashMap;
use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData};

pub trait NeuronXYZPEncoder {
    /// Returns the I/O data type this encoder handles.
    ///
    /// This method specifies which [`IOTypeVariant`] this encoder is designed
    /// to process. The type system uses this for validation and routing.
    ///
    /// # Returns
    ///
    /// The [`IOTypeVariant`] that this encoder can process.
    fn get_input_data_type(&self) -> IOTypeVariant;
    
    fn get_channel_dimensions(&self) -> &SingleChannelDimensions;
    
    fn get_cortical_id_write_destinations(&self) -> &[CorticalID];
    
    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError>;

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
    fn write_neuron_data_multi_channel(&self, channels_and_values: HashMap<CorticalIOChannelIndex, &IOTypeData>, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        for (channel, values) in channels_and_values {
            self.write_neuron_data_single_channel(values, channel, write_target)?;
        };
        Ok(())
    }
}

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

    fn get_channel_dimensions(&self) -> &SingleChannelDimensions;

    fn get_cortical_id_read_destinations(&self) -> &[CorticalID];
    
    fn read_neuron_data_single_channel(&self, cortical_channel: CorticalIOChannelIndex, read_from: &CorticalMappedXYZPNeuronData) -> Result<IOTypeData, FeagiDataProcessingError>;

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
    fn read_neuron_data_multi_channel(&self, channels: &[CorticalIOChannelIndex],  read_from: &CorticalMappedXYZPNeuronData) -> Result<Vec<IOTypeData>, FeagiDataProcessingError> {
        let mut output: Vec<IOTypeData> = Vec::with_capacity(channels.len());
        for channel in channels {
            output.push(self.read_neuron_data_single_channel(*channel, read_from)?);
        };
        Ok(output)
    }
}