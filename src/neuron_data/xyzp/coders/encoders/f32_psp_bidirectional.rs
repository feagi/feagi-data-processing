use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZP, NeuronXYZPArrays};
use crate::neuron_data::xyzp::coders::NeuronXYZPEncoder;
pub(crate) struct F32PSPBidirectionalNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: CorticalID
}

impl NeuronXYZPEncoder for F32PSPBidirectionalNeuronXYZPEncoder {

    fn get_encodable_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32NormalizedM1To1
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types

        let value: f32 = wrapped_value.try_into()?;

        const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;

        let generated_neuron_data: &mut NeuronXYZPArrays = write_target.ensure_clear_and_borrow_mut(&self.cortical_write_target, NUMBER_NEURONS_IN_STRUCTURE);
        let channel_offset: u32 = self.channel_dimensions.get_x() * *cortical_channel + { if value.is_sign_positive() { 1 } else { 0 } };
        let p_value: f32 = value.into();
        let neuron: NeuronXYZP = NeuronXYZP::new(
            channel_offset,
            0,
            0,
            p_value.abs()
        );
        generated_neuron_data.add_neuron(&neuron);
        Ok(())
    }
}

impl F32PSPBidirectionalNeuronXYZPEncoder {
    pub fn new(cortical_write_target: CorticalID, channel_dimensions: SingleChannelDimensions) -> Self {
        F32PSPBidirectionalNeuronXYZPEncoder {
            channel_dimensions,
            cortical_write_target
        }
    }
}