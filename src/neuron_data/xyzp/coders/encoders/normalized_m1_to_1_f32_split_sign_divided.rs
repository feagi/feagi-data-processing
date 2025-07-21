use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::io_data::{IOTypeData, NormalizedM1To1F32};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZP, NeuronXYZPArrays};
use crate::neuron_data::xyzp::coders::NeuronXYZPEncoder;

pub struct NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: CorticalID
}

impl NeuronXYZPEncoder for NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder {
    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types

        let value: NormalizedM1To1F32 = wrapped_value.try_into()?;

        const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;

        let generated_neuron_data: &mut NeuronXYZPArrays = write_target.ensure_clear_and_borrow_mut(&self.cortical_write_target, NUMBER_NEURONS_IN_STRUCTURE);
        let channel_offset: u32 = self.channel_dimensions.get_x() * *cortical_channel + { if value.is_sign_positive() { 1 } else { 0 } };
        let p_val: f32 = value.into();
        let neuron: NeuronXYZP = NeuronXYZP::new(
            channel_offset,
            0,
            0,
            p_val.abs()
        );
        generated_neuron_data.add_neuron(&neuron);
        Ok(())
    }
}

impl NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder {
    pub fn new(cortical_write_target: CorticalID, channel_dimensions: SingleChannelDimensions) -> Self {
        NormalizedM1to1F32FloatSplitSignDividedNeuronXYZPEncoder {
            channel_dimensions,
            cortical_write_target,
        }
    }
}