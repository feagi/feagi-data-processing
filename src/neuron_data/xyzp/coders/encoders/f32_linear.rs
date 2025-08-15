use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::processors::LinearScaleTo0And1;
use crate::io_processing::StreamCacheProcessor;
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
        generated_neuron_data.push(&neuron);
        Ok(())
    }

    fn create_default_processor_chain(&self) -> Vec<Box<dyn StreamCacheProcessor + Sync + Send>> {
        vec![Box::new(LinearScaleTo0And1::new(0f32, 1f32, 0f32).unwrap())]
    }
}

impl F32LinearNeuronXYZPEncoder {
    
    pub const CHANNEL_X_LENGTH: u32 = 1;
    pub const CHANNEL_Y_LENGTH: u32 = 1;
    
    pub fn new(cortical_write_target: CorticalID, z_resolution: u32) -> Result<Self, FeagiDataProcessingError> {
        
        Ok(F32LinearNeuronXYZPEncoder {
            channel_dimensions: SingleChannelDimensions::new(Self::CHANNEL_X_LENGTH, Self::CHANNEL_Y_LENGTH, z_resolution)?,
            cortical_write_target,
            z_res: z_resolution as f32,
        })
    }
}