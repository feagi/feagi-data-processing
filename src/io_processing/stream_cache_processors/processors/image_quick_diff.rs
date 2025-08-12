use std::fmt::Display;
use std::time::Instant;
use ndarray::{Array3, Zip};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{IOTypeData, IOTypeVariant, ImageFrame};
use crate::io_data::image_descriptors::ImageFrameProperties;
use crate::io_processing::StreamCacheProcessor;

#[derive(Debug)]
pub struct ImageFrameQuickDiffProcessor {
    diff_cache: IOTypeData, // Image Frame
    cached_a: IOTypeData, // Image Frame
    cached_b: IOTypeData, // Image Frame
    input_definition: ImageFrameProperties,
    is_diffing_against_b: bool,
    threshold: f32,
}

impl Display for ImageFrameQuickDiffProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageFrameQuickDiffProcessor()")
    }
}

impl StreamCacheProcessor for ImageFrameQuickDiffProcessor {
    fn get_input_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.input_definition))
    }

    fn get_output_data_type(&self) -> IOTypeVariant {
        IOTypeVariant::ImageFrame(Some(self.input_definition))
    }

    fn get_most_recent_output(&self) -> &IOTypeData {
        &self.diff_cache
    }

    fn process_new_input(&mut self, value: &IOTypeData, _time_of_input: Instant) -> Result<&IOTypeData, FeagiDataProcessingError> {
        if self.is_diffing_against_b {
            self.cached_a = value.clone();
            quick_diff(&self.cached_a, &self.cached_b, &mut self.diff_cache, self.threshold);
        }
        else {
            self.cached_b = value.clone();
            quick_diff(&self.cached_b, &self.cached_a, &mut self.diff_cache, self.threshold);
        }
        self.is_diffing_against_b = !self.is_diffing_against_b;
        Ok(&self.diff_cache)
    }
}

impl ImageFrameQuickDiffProcessor {
    pub fn new(image_properties: ImageFrameProperties, threshold: f32) -> Result<Self, FeagiDataProcessingError> {
        if threshold < 0.0 {
            return Err(IODataError::InvalidParameters("Threshold must be positive!".into()).into());
        }
        
        let cache_image = ImageFrame::from_image_frame_properties(&image_properties)?;
        Ok(ImageFrameQuickDiffProcessor {
            diff_cache: IOTypeData::ImageFrame(cache_image.clone()),
            cached_a: IOTypeData::ImageFrame(cache_image.clone()), // Image Frame
            cached_b: IOTypeData::ImageFrame(cache_image.clone()), // Image Frame
            input_definition: image_properties,
            is_diffing_against_b: false,
            threshold,
        })
    }
}

fn quick_diff(source: &IOTypeData, source_diffing: &IOTypeData, diff_overwriting: &mut IOTypeData, threshold: f32) -> Result<(), FeagiDataProcessingError> {
    let read_from: &ImageFrame = source.try_into()?;
    let source_diff_from: &ImageFrame = source_diffing.try_into()?;
    let write_to: &mut ImageFrame = diff_overwriting.try_into()?;

    let read_from: &Array3<f32> = read_from.get_internal_data();
    let source_diff_from: &Array3<f32> = source_diff_from.get_internal_data();
    let write_to: &mut Array3<f32> = write_to.get_internal_data_mut();
    
    Zip::from(write_to).and(read_from).and(source_diff_from).for_each(|w, &r, &s| {
        let x = r - s;
        if x > threshold {
            *w = r;
        }
        else {
            *w = 0f32;
        }
    });
    
    Ok(())
}

