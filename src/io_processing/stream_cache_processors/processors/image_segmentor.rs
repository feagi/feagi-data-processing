use std::fmt::Display;
use std::time::Instant;
use crate::error::FeagiDataProcessingError;
use crate::io_data::image_descriptors::{ImageFrameProperties, SegmentedFrameCenterProperties};
use crate::io_data::{IOTypeData, IOTypeVariant};
use crate::io_processing::StreamCacheProcessor;

#[derive(Debug)]
pub struct ImageFrameSegmentatorProcessor {
    input_image_properties: ImageFrameProperties,
    output_image_properties: [ImageFrameProperties; 9],
    image_segmentor: SegmentedFrameCenterProperties,
    cached: IOTypeData
}

impl Display for ImageFrameSegmentatorProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageFrameSegmentatorProcessor()") // TODO fill out
    }
}

impl StreamCacheProcessor for ImageFrameSegmentatorProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.input_image_properties))
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::SegmentedImageFrame(Some(self.output_image_properties))
    }

    fn get_most_recent_output(&self) -> &IOTypeData { &self.cached }

    fn process_new_input(&mut self, value: &IOTypeData, time_of_input: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        todo!()
    }
}




