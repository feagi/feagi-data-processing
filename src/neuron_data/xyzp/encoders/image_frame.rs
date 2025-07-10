use crate::error::{FeagiDataProcessingError, IODataError, NeuronError};
use crate::genomic_structures::{CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType, SingleChannelDimensions};
use crate::neuron_data::xyzp::{NeuronXYZPEncoder, NeuronXYZP, CorticalMappedXYZPNeuronData, NeuronXYZPArrays};
use crate::io_data::{ImageFrame, IOTypeData, IOTypeVariant};

pub struct ImageFrameNeuronXYZPEncoder {
    single_cortical_id: [CorticalID; 1],
    expected_image_resolution_per_channel: (usize, usize),
    channel_count: u32,
    threshold: f32
}

impl NeuronXYZPEncoder for ImageFrameNeuronXYZPEncoder {
    fn get_data_type() -> IOTypeVariant {
        IOTypeVariant::ImageFrame
    }

    fn get_cortical_ids_writing_to(&self) -> &[CorticalID] {
        &self.single_cortical_id
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        
        if *cortical_channel > self.channel_count {
            return Err(FeagiDataProcessingError::from(NeuronError::UnableToGenerateNeuronData(format!("Requested channel {} is not supported when max channel is {}!", *cortical_channel, self.channel_count))).into());
        }

        if wrapped_value.variant() != Self::get_data_type() {
            return Err(NeuronError::UnableToGenerateNeuronData(format!("Given sensor value is not {}! Instead received type {}!", Self::get_data_type().to_string(), wrapped_value.variant().to_string())).into());
        }
        
        let mut image: ImageFrame = wrapped_value.try_into().unwrap();
        if image.get_cartesian_width_height() != self.expected_image_resolution_per_channel {
            return Err(IODataError::InvalidParameters("Input impage does not have the expected resolution!".into()).into());
        }
        
        let cortical_id: &CorticalID = &self.single_cortical_id[0];
        let max_number_neurons_needed = image.get_max_capacity_neuron_count(); // likely over allocating, but since there should be no further allocations (memory reuse), we should be fine
        let generated_neuron_data: &mut NeuronXYZPArrays =  write_target.ensure_clear_and_borrow_mut(cortical_id, max_number_neurons_needed);
        image.write_thresholded_xyzp_neuron_arrays(self.threshold, generated_neuron_data)?;
        
        Ok(())
    }
}

impl ImageFrameNeuronXYZPEncoder {
    pub fn new(number_channels: u32, cortical_type: CorticalType, cortical_index: CorticalGroupingIndex, threshold: f32, expected_image_resolution_per_channel: (usize, usize)) -> Result<Self, FeagiDataProcessingError> {
        cortical_type.verify_valid_io_variant(&Self::get_data_type())?;
        let cortical_id = CorticalID::try_from_cortical_type(&cortical_type, cortical_index)?;
        Ok(ImageFrameNeuronXYZPEncoder{
            single_cortical_id: [cortical_id],
            channel_count: number_channels,
            expected_image_resolution_per_channel,
            threshold,
        })
    }
}