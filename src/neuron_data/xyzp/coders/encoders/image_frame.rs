use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPArrays};
use crate::neuron_data::xyzp::coders::NeuronXYZPEncoder;
use crate::io_data::{ImageFrame, IOTypeData, IOTypeVariant};

pub(crate) struct ImageFrameNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: [CorticalID; 1]
}

impl NeuronXYZPEncoder for ImageFrameNeuronXYZPEncoder {

    fn get_encodable_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(None) // Any Image frame can be processed
    }
    
    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types

        let image: &ImageFrame = wrapped_value.try_into()?;
        let cortical_id: &CorticalID = &self.cortical_write_target[0];
        
        // TODO image size check
        
        let max_number_neurons_needed = image.get_max_capacity_neuron_count(); // likely over allocating, but since there should be no further allocations (memory reuse), we should be fine
        let generated_neuron_data: &mut NeuronXYZPArrays =  write_target.ensure_clear_and_borrow_mut(cortical_id, max_number_neurons_needed);
        image.write_xyzp_neuron_arrays(generated_neuron_data, cortical_channel)?;
        Ok(())
    }
}

impl ImageFrameNeuronXYZPEncoder {
    pub fn new(cortical_write_target: CorticalID, channel_dimensions: SingleChannelDimensions) -> Self {        
        ImageFrameNeuronXYZPEncoder{
            channel_dimensions,
            cortical_write_target: [cortical_write_target; 1]
        }
    }
}