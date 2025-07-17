use crate::error::{FeagiDataProcessingError, IODataError};
use crate::neuron_data::xyzp::{NeuronXYZPEncoder, NeuronXYZP, CorticalMappedXYZPNeuronData, NeuronXYZPArrays};
use crate::io_data::{IOTypeData, IOTypeVariant, LinearM1to1NormalizedF32};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};

pub struct LinearNormalizedFloatSplitSignDividedNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: [CorticalID; 1]
}

impl NeuronXYZPEncoder for LinearNormalizedFloatSplitSignDividedNeuronXYZPEncoder {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::LinearM1to1NormalizedF32
    }

    fn get_channel_dimensions(&self) -> &SingleChannelDimensions {
        &self.channel_dimensions
    }

    fn get_cortical_id_write_destinations(&self) -> &[CorticalID] {
        &self.cortical_write_target
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types

        let value: LinearM1to1NormalizedF32 = wrapped_value.try_into()?;

        const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;
        
        let generated_neuron_data: &mut NeuronXYZPArrays = write_target.ensure_clear_and_borrow_mut(&self.cortical_write_target[0], NUMBER_NEURONS_IN_STRUCTURE);
        let channel_offset: u32 = self.channel_dimensions.get_x() * *cortical_channel + { if value.is_sign_positive() { 1 } else { 0 } };
        let neuron: NeuronXYZP = NeuronXYZP::new(
            channel_offset,
            0,
            0,
            value.asf32().abs()
        );
        generated_neuron_data.add_neuron(&neuron);
        Ok(())
    }
}

impl LinearNormalizedFloatSplitSignDividedNeuronXYZPEncoder {
    pub fn new(cortical_id_target: CorticalID, z_resolution: u32) -> Result<Self, FeagiDataProcessingError> {
        if z_resolution == 0 {
            return Err(IODataError::InvalidParameters("Resolution cannot be 0!".into()).into());
        }
        pub const CHANNEL_WIDTH: u32 = 2;
        pub const CHANNEL_HEIGHT: u32 = 1;
        Ok(LinearNormalizedFloatSplitSignDividedNeuronXYZPEncoder{
            channel_dimensions: SingleChannelDimensions::new(CHANNEL_WIDTH, CHANNEL_HEIGHT, z_resolution)?,
            cortical_write_target: [cortical_id_target; 1]
        })
        
    }
}



pub struct LinearNormalizedFloatPSPBirdirectionalNeuronXYZPEncoder {
    channel_dimensions: SingleChannelDimensions,
    cortical_write_target: [CorticalID; 1]
}

// TODO Right now this is a clone of the above. This is technically correct but we can make it nicer
impl NeuronXYZPEncoder for LinearNormalizedFloatPSPBirdirectionalNeuronXYZPEncoder {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::LinearM1to1NormalizedF32
    }

    fn get_channel_dimensions(&self) -> &SingleChannelDimensions {
        &self.channel_dimensions
    }

    fn get_cortical_id_write_destinations(&self) -> &[CorticalID] {
        self.cortical_write_target.as_ref()
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types

        let value: LinearM1to1NormalizedF32 = wrapped_value.try_into()?;

        const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;

        let generated_neuron_data: &mut NeuronXYZPArrays = write_target.ensure_clear_and_borrow_mut(&self.cortical_write_target[0], NUMBER_NEURONS_IN_STRUCTURE);
        let channel_offset: u32 = self.channel_dimensions.get_x() * *cortical_channel + { if value.is_sign_positive() { 1 } else { 0 } };
        let neuron: NeuronXYZP = NeuronXYZP::new(
            channel_offset,
            0,
            0,
            value.asf32().abs()
        );
        generated_neuron_data.add_neuron(&neuron);
        Ok(())
    }
}

impl LinearNormalizedFloatPSPBirdirectionalNeuronXYZPEncoder {
    pub fn new(cortical_id_target: CorticalID) -> Result<Self, FeagiDataProcessingError> {
        pub const CHANNEL_WIDTH: u32 = 2;
        pub const CHANNEL_HEIGHT: u32 = 1;
        pub const CHANNEL_DEPTH: u32 = 1;
        
        Ok(LinearNormalizedFloatPSPBirdirectionalNeuronXYZPEncoder{
            channel_dimensions: SingleChannelDimensions::new(CHANNEL_WIDTH, CHANNEL_HEIGHT, CHANNEL_DEPTH)?,
            cortical_write_target: [cortical_id_target; 1]
        })

    }
}