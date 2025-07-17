use crate::error::{FeagiDataProcessingError, IODataError, NeuronError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, CorticalType, SingleChannelDimensions};
use crate::neuron_data::xyzp::{NeuronXYZPEncoder, CorticalMappedXYZPNeuronData, NeuronXYZPArrays};
use crate::io_data::{ImageFrame, IOTypeData, IOTypeVariant};
use crate::io_data::descriptors::ChannelFormat;

pub struct ImageFrameNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: [CorticalID; 1]
}

impl NeuronXYZPEncoder for ImageFrameNeuronXYZPEncoder {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame
    }

    fn get_channel_dimensions(&self) -> &SingleChannelDimensions {
        &self.channel_dimensions
    }

    fn get_cortical_id_write_destinations(&self) -> &[CorticalID] {
        &self.cortical_write_target
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types

        let mut image: &ImageFrame = wrapped_value.try_into()?;
        let cortical_id: &CorticalID = &self.cortical_write_target[0];
        let max_number_neurons_needed = image.get_max_capacity_neuron_count(); // likely over allocating, but since there should be no further allocations (memory reuse), we should be fine
        let generated_neuron_data: &mut NeuronXYZPArrays =  write_target.ensure_clear_and_borrow_mut(cortical_id, max_number_neurons_needed);
        image.write_xyzp_neuron_arrays(generated_neuron_data, cortical_channel)?;
        Ok(())
    }
}

impl ImageFrameNeuronXYZPEncoder {
    pub fn new(cortical_id_target: CorticalID, image_cartesian_width_height: (u32, u32), channel_format: ChannelFormat) -> Result<Self, FeagiDataProcessingError> {
        let channel_dimensions: SingleChannelDimensions = SingleChannelDimensions::new(
            image_cartesian_width_height.0,
            image_cartesian_width_height.1,
            channel_format as u32
        )?;
        
        Ok(ImageFrameNeuronXYZPEncoder{
            channel_dimensions,
            cortical_write_target: [cortical_id_target; 1]
        })
    }
}