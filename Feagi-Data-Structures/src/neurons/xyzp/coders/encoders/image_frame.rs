use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalID, CorticalIOChannelIndex, SingleChannelDimensions};
use crate::neuron_data::xyzp::{CorticalMappedXYZPNeuronData, NeuronXYZPArrays};
use crate::neuron_data::xyzp::coders::NeuronXYZPEncoder;
use crate::io_data::{ImageFrame, IOTypeData, IOTypeVariant};
use crate::io_data::image_descriptors::ImageFrameProperties;

pub(crate) struct ImageFrameNeuronXYZPEncoder {
    image_properties: ImageFrameProperties,
    cortical_write_target: CorticalID
}

impl NeuronXYZPEncoder for ImageFrameNeuronXYZPEncoder {

    fn get_encodable_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.image_properties))
    }
    
    fn write_neuron_data_single_channel(&self, wrapped_value: &IOTypeData, cortical_channel: CorticalIOChannelIndex, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        // We are not doing any sort of verification checks here, other than ensuring data types
        let image: &ImageFrame = wrapped_value.try_into()?;
        image.write_as_neuron_xyzp_data(write_target, self.cortical_write_target, cortical_channel)?;
        Ok(())
    }
}

impl ImageFrameNeuronXYZPEncoder {
    pub fn new(cortical_write_target: CorticalID, image_properties: &ImageFrameProperties) -> Result<Self, FeagiDataProcessingError> {        
        Ok(ImageFrameNeuronXYZPEncoder{
            image_properties: image_properties.clone(),
            cortical_write_target
        })
    }
}