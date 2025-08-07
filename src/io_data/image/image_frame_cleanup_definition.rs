use crate::error::FeagiDataProcessingError;
use crate::io_data::image_descriptors::{ColorSpace, CornerPoints, MemoryOrderLayout};

pub struct ImageFrameCleanupDefinition {
    input_xy_resolution: (usize, usize),
    cropping_from: Option<CornerPoints>,
    final_resize_xy_to: Option<(usize, usize)>,
    multiply_brightness_by: Option<f32>,
    change_contrast_by: Option<f32>,
    convert_color_space_to: Option<ColorSpace>,
    convert_to_grayscale: bool
}

impl std::fmt::Display for ImageFrameCleanupDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut steps: String = match (self.cropping_from, self.final_resize_xy_to) {
            (None, None) => format!("Keeping input size of <{}, {}> (no cropping from or resizing to)", self.input_xy_resolution.0, self.input_xy_resolution.1),
            (Some(cropping_from), None) => format!("Cropping from xy points <{}, {}> to <{}, {}> without resizing after,", 
                                                   cropping_from.lower_left_row_major().1, cropping_from.lower_left_row_major().0, cropping_from.upper_right_row_major().1, cropping_from.upper_right_row_major().0),
            (None, Some(final_resize_xy_to)) => format!("resizing to resolution <{}, {}> without any cropping,", final_resize_xy_to.0, final_resize_xy_to.1),
            (Some(cropping_from), Some(final_resize_xy_to)) => format!("Cropping from xy points <{}, {}> to <{}, {}> then resizing to resolution <{}, {}>,", 
                                                                       cropping_from.lower_left_row_major().1, cropping_from.lower_left_row_major().0, cropping_from.upper_right_row_major().1, cropping_from.upper_right_row_major().0, final_resize_xy_to.0, final_resize_xy_to.1),
        };
        steps += &*(match self.multiply_brightness_by {
            None => String::new(),
            Some(multiply_brightness_by) => format!("Multiply brightness by {}", multiply_brightness_by),
        });
        steps += &*(match self.change_contrast_by {
            None => String::new(),
            Some(change_contrast_by) => format!("Change contrast by {}", change_contrast_by),
        });
        steps += &*(match self.convert_color_space_to {
            None => String::new(),
            Some(change_colorspace_to) => format!("Convert Colorspace to {}", change_colorspace_to.to_string()),
        });
        steps += &*(match self.convert_to_grayscale {
            false => String::new(),
            true => "Convert to grayscale".to_string(),
        });
        write!(f, "ImageFrameCleanupDefinition({})", steps)
    }
}

impl ImageFrameCleanupDefinition {
    pub fn new(input_xy_resolution: (usize, usize)) -> ImageFrameCleanupDefinition {
        ImageFrameCleanupDefinition{
            input_xy_resolution,
            cropping_from: None,
            final_resize_xy_to: None,
            multiply_brightness_by: None,
            change_contrast_by: None,
            convert_color_space_to: None,
            convert_to_grayscale: false
        }
    }
    
    //region set settings
    // TODO create clear all, clear individual settings
    
    pub fn set_cropping_from(&mut self, lower_left_xy_point_inclusive: (usize, usize), upper_right_xy_point_exclusive: (usize, usize)) -> Result<&Self, FeagiDataProcessingError> {
        let corner_points = CornerPoints::new_from_cartesian(lower_left_xy_point_inclusive, upper_right_xy_point_exclusive, self.input_xy_resolution)?;
        self.cropping_from = Some(corner_points);
        Ok(self)
    }
    
    pub fn set_resizing_to(&mut self, new_xy_resolution: (usize, usize)) -> Result<&Self, FeagiDataProcessingError> {
        self.final_resize_xy_to = Some(new_xy_resolution);
        Ok(self)
    }
    
    pub fn set_brightness_multiplier(&mut self, brightness_multiplier: f32) -> Result<&Self, FeagiDataProcessingError> {
        self.multiply_brightness_by = Some(brightness_multiplier);
        Ok(self)
    }
    
    pub fn set_contrast_change(&mut self, contrast_change: f32) -> Result<&Self, FeagiDataProcessingError> {
        self.change_contrast_by = Some(contrast_change);
        Ok(self)
    }
    
    pub fn set_color_space_to(&mut self, color_space: ColorSpace) -> Result<&Self, FeagiDataProcessingError> {
        self.convert_color_space_to = Some(color_space);
        Ok(self)
    }
    
    pub fn set_conversion_to_grayscale(&mut self) -> Result<&Self, FeagiDataProcessingError> {
        self.convert_to_grayscale = true;
        Ok(self)
    }
    
    //endregion
    
    
    
    
}