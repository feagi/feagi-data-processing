use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex};
use crate::io_data::{IOTypeData, IOTypeVariant, SegmentedImageFrame};
use crate::io_data::image_descriptors::ImageFrameProperties;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};

pub(crate) struct SegmentedImageFrameNeuronXYZPEncoder {
    image_frames_properties: [ImageFrameProperties; 9],
    cortical_write_targets: [CorticalID; 9],
}

impl NeuronXYZPEncoder for SegmentedImageFrameNeuronXYZPEncoder {
    fn get_encodable_data_type(&self) -> IOTypeVariant {
        // Since changing Image Frame Properties often mean changing channel size, we shouldn't allow doing that
        IOTypeVariant::SegmentedImageFrame(Some(self.image_frames_properties))
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types
        let segmented_image: &SegmentedImageFrame = wrapped_value.try_into()?;
        segmented_image.write_as_neuron_xyzp_data(write_target, cortical_channel, &self.cortical_write_targets)?;
        Ok(())
    }
}

impl SegmentedImageFrameNeuronXYZPEncoder {
    pub fn new(cortical_write_targets: [CorticalID; 9], image_properties: [ImageFrameProperties; 9]) -> Result<Self, FeagiDataProcessingError> {
        Ok(SegmentedImageFrameNeuronXYZPEncoder{
            image_frames_properties: image_properties,
            cortical_write_targets
        })
    }
}