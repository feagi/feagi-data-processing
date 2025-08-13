use crate::error::FeagiDataProcessingError;
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex};
use crate::io_data::{IOTypeData, IOTypeVariant, SegmentedImageFrame};
use crate::io_data::image_descriptors::ImageFrameProperties;
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPEncoder};
use crate::neuron_data::xyzp::coders::encoders::ImageFrameNeuronXYZPEncoder;

pub(crate) struct SegmentedImageFrameNeuronXYZPEncoder {
    image_encoders: [ImageFrameNeuronXYZPEncoder; 9]
}

impl NeuronXYZPEncoder for SegmentedImageFrameNeuronXYZPEncoder {
    fn get_encodable_data_type(&self) -> IOTypeVariant {
        // Since changing Image Frame Properties often mean changing channel size, we shouldn't allow doing that
        IOTypeVariant::SegmentedImageFrame(Some(self.image_frames_properties))
    }

    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types
        
        let segmented_image: &SegmentedImageFrame = wrapped_value.try_into()?;
        
        
        todo!()
    }
}

impl SegmentedImageFrame {
    pub fn new() -> Self {}
}