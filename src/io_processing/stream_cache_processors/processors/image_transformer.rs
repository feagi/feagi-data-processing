use std::fmt::Display;
use std::time::Instant;
use crate::error::FeagiDataProcessingError;
use crate::io_data::{IOTypeData, IOTypeVariant, ImageFrame, ImageFrameTransformerDefinition};
use crate::io_processing::StreamCacheProcessor;

#[derive(Debug)]
pub struct ImageFrameTransformerProcessor {
    transformer_definition: ImageFrameTransformerDefinition,
    cached: IOTypeData, // Image Frame
}

impl Display for ImageFrameTransformerProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageFrameTransformerProcessor({:?})", self.transformer_definition)
    }
}

impl StreamCacheProcessor for ImageFrameTransformerProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(*self.transformer_definition.get_input_image_properties()))
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.transformer_definition.get_output_image_properties()))
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.cached
    }

    fn process_new_input(&mut self, value: &IOTypeData, _time_of_input: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        let read_from: &ImageFrame = value.try_into()?;
        let write_target: &mut ImageFrame = (&mut self.cached).try_into()?;
        self.transformer_definition.process_image(read_from, write_target)?;
        Ok(&self.cached)
    }
}

impl ImageFrameTransformerProcessor {
    pub fn new(transformer_definition: ImageFrameTransformerDefinition) -> Result<Self, FeagiDataProcessingError> {
        Ok(ImageFrameTransformerProcessor{
            cached: IOTypeData::ImageFrame(ImageFrame::from_image_frame_properties(&transformer_definition.get_output_image_properties())?),
            transformer_definition,
        })
    }
}