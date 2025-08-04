use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZP, NeuronXYZPArrays};
use super::super::{NeuronXYZPEncoder};

pub(crate) struct F32LinearNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: CorticalID,
    z_res: f32,
}

impl NeuronXYZPEncoder for F32LinearNeuronXYZPEncoder {
    fn get_encodable_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::F32Normalized0To1
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types
        
        let value: f32 = wrapped_value.try_into()?;
        
        let z_dist: f32 = value * self.z_res;
        let channel_offset: u32 = self.channel_dimensions.get_x() * *cortical_channel;
        let z_index: u32 = z_dist.floor() as u32;
        let neuron: NeuronXYZP = NeuronXYZP::new(
            channel_offset,
            0,
            z_index,
            1.0
        );

        const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;
        let generated_neuron_data: &mut NeuronXYZPArrays = write_target.ensure_clear_and_borrow_mut(&self.cortical_write_target, NUMBER_NEURONS_IN_STRUCTURE);
        generated_neuron_data.add_neuron(&neuron);
        Ok(())
    }
}

impl F32LinearNeuronXYZPEncoder {
    pub fn new(cortical_write_target: CorticalID, channel_dimensions: SingleChannelDimensions) -> Self {
        let z_res: f32 = channel_dimensions.get_z() as f32;
        F32LinearNeuronXYZPEncoder {
            channel_dimensions,
            cortical_write_target,
            z_res
        }
    }
}