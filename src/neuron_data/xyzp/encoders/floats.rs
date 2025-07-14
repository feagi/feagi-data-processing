use crate::error::{FeagiDataProcessingError, NeuronError};
use crate::neuron_data::xyzp::{NeuronXYZPEncoder, NeuronXYZP, CorticalMappedXYZPNeuronData, NeuronXYZPArrays};
use crate::io_data::{IOTypeData, IOTypeVariant, LinearNormalizedF32};
use crate::genomic_structures::{CorticalAreaDimensions, CorticalGroupingIndex, CorticalID, CorticalIOChannelIndex, CorticalType};
use crate::neuron_data::neuron_layouts::FloatNeuronLayoutType;

// TODO we may need cortical dimensions here or channel dimensions at some point

pub struct LinearNormalizedFloatNeuronXYZPEncoder {
    translator_type: FloatNeuronLayoutType,
    single_cortical_id: [CorticalID; 1],
    channel_count: u32
}

impl NeuronXYZPEncoder for LinearNormalizedFloatNeuronXYZPEncoder {

    fn get_encoded_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::LinearNormalizedFloat
    }


    fn get_cortical_ids_writing_to(&self) -> &[CorticalID] {
        &self.single_cortical_id
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        if *cortical_channel > self.channel_count {
            return Err(FeagiDataProcessingError::from(NeuronError::UnableToGenerateNeuronData(format!("Requested channel {} is not supported when max channel is {}!", *cortical_channel, self.channel_count))).into());
        }

        if wrapped_value.variant() != IOTypeVariant::LinearNormalizedFloat {
            return Err(NeuronError::UnableToGenerateNeuronData(format!("Given sensor value is not {}! Instead received type {}!", self.get_encoded_data_type().to_string(), wrapped_value.variant().to_string())).into());
        }

        let value: LinearNormalizedF32 = wrapped_value.try_into().unwrap();
        let cortical_id: &CorticalID = &self.single_cortical_id[0];
        let translator_type = &self.translator_type;

        let generated_neuron_data: &mut NeuronXYZPArrays;
        match translator_type {
            FloatNeuronLayoutType::PSPBidirectional => {
                const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;
                generated_neuron_data = write_target.ensure_clear_and_borrow_mut(cortical_id, NUMBER_NEURONS_IN_STRUCTURE);
                let channel_offset: u32 = FloatNeuronLayoutType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL * *cortical_channel + { if value.is_sign_positive() { 1 } else { 0 } };
                let neuron: NeuronXYZP = NeuronXYZP::new(
                    channel_offset,
                    0,
                    0,
                    value.asf32().abs()
                );
                generated_neuron_data.add_neuron(&neuron);
                Ok(())
            },

            FloatNeuronLayoutType::SplitSignDivided => {
                // TODO Right now we are using the same algo as PSPBidirectional which works, but wouldn't it look nicer to use something that uses the full bounds?
                const NUMBER_NEURONS_IN_STRUCTURE: usize = 1;
                generated_neuron_data = write_target.ensure_clear_and_borrow_mut(cortical_id, NUMBER_NEURONS_IN_STRUCTURE);
                let channel_offset: u32 = FloatNeuronLayoutType::CHANNEL_WIDTH_PSP_BIDIRECTIONAL * *cortical_channel + { if value.is_sign_positive() { 1 } else { 0 } };
                let neuron: NeuronXYZP = NeuronXYZP::new(
                    channel_offset,
                    0,
                    0,
                    value.asf32().abs()
                );
                generated_neuron_data.add_neuron(&neuron);
                Ok(())
            },

            FloatNeuronLayoutType::Linear => {
                Err(FeagiDataProcessingError::NotImplemented)
            }
        }
    }
}

impl LinearNormalizedFloatNeuronXYZPEncoder {
    pub fn new(number_channels: u32, cortical_type: CorticalType, cortical_index: CorticalGroupingIndex, translator_type: FloatNeuronLayoutType) -> Result<Self, FeagiDataProcessingError> {
        cortical_type.verify_valid_io_variant(&IOTypeVariant::LinearNormalizedFloat)?;
        let cortical_id = CorticalID::try_from_cortical_type(&cortical_type, cortical_index)?;
        Ok(LinearNormalizedFloatNeuronXYZPEncoder {
            translator_type,
            single_cortical_id: [cortical_id],
            channel_count: number_channels
        })
    }
}