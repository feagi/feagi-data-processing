use ndarray::{s, Array3, ArrayView3};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::image_descriptors::{ChannelLayout, ColorSpace, CornerPoints, ImageFrameProperties, MemoryOrderLayout};
use crate::io_data::ImageFrame;

pub struct ImageFrameCleanupDefinition {
    input_image_properties: ImageFrameProperties,
    cropping_from: Option<CornerPoints>,
    final_resize_xy_to: Option<(usize, usize)>,
    convert_color_space_to: Option<ColorSpace>,
    multiply_brightness_by: Option<f32>,
    change_contrast_by: Option<f32>,
    convert_to_grayscale: bool,
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
        steps += &*(match self.convert_color_space_to {
            None => String::new(),
            Some(change_colorspace_to) => format!("Convert Colorspace to {}", change_colorspace_to.to_string()),
        });
        steps += &*(match self.multiply_brightness_by {
            None => String::new(),
            Some(multiply_brightness_by) => format!("Multiply brightness by {}", multiply_brightness_by),
        });
        steps += &*(match self.change_contrast_by {
            None => String::new(),
            Some(change_contrast_by) => format!("Change contrast by {}", change_contrast_by),
        });
        steps += &*(match self.convert_to_grayscale {
            false => String::new(),
            true => "Convert to grayscale".to_string(),
        });
        write!(f, "ImageFrameCleanupDefinition({})", steps)
    }
}

impl ImageFrameCleanupDefinition {
    pub fn new(input_image_properties: ImageFrameProperties) -> ImageFrameCleanupDefinition {
        ImageFrameCleanupDefinition{
            input_image_properties,
            cropping_from: None,
            final_resize_xy_to: None,
            multiply_brightness_by: None,
            change_contrast_by: None,
            convert_color_space_to: None,
        }
    }

    pub fn get_output_image_properties(&self) -> ImageFrameProperties {
        let resolution = match (self.cropping_from, self.final_resize_xy_to) {
            (None, None) => self.input_image_properties.get_expected_xy_resolution(),
            (Some(cropping_from), None) => self.cropping_from.unwrap().enclosed_area_width_height(),
            (None, Some(final_resize_xy_to)) => final_resize_xy_to,
            (Some(_), Some(final_resize_xy_to)) => final_resize_xy_to,
        };
        let color_space = match self.convert_color_space_to {
            None => self.input_image_properties.get_expected_color_space(),
            Some(color_space_to) => color_space_to,
        };
        let color_channel_layout = match self.convert_to_grayscale {
            false => self.input_image_properties.get_expected_color_channel_layout(),
            true => ChannelLayout::GrayScale,
        };
        ImageFrameProperties::new(resolution, color_space, color_channel_layout)
    }



    //region set settings
    // TODO create clear all, clear individual settings

    // TODO safety bound checks!

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

    //region image processing

    pub(crate) fn process_image(&self, source: &ImageFrame, destination: &mut ImageFrame) -> Result<(), FeagiDataProcessingError> {
        // TODO build optimal routes!
        // TODO right now we are only cropping and resizing
        
        // Due to image segmentor, I would argue the most common route is crop + resize + grayscale
        
        if self.cropping_from.is_some() && self.final_resize_xy_to.is_some() {
            if self.convert_color_space_to.is_none() && self.multiply_brightness_by.is_none() && self.change_contrast_by.is_none() {
                self.crop_and_resize(source, destination);
                return Ok(());
            }
            
            
        }
            
        if self.cropping_from.is_some() && self.final_resize_xy_to.is_none() {
            
        }

        if self.cropping_from.is_none() && self.final_resize_xy_to.is_some() {

        }
        
        // no cropping or resizing

    }
    
    fn get_output_channel_count(&self) -> usize {
        if self.convert_to_grayscale {
            return 1;
        }
        self.input_image_properties.get_expected_color_channel_layout().into()
    }
    
    fn crop(&self, source: &ImageFrame, destination: &mut ImageFrame, number_color_channels: usize) -> Result<(), FeagiDataProcessingError> {
        let crop_from = self.cropping_from.unwrap();
        // TODO whats going on with color channels here
        let mut destination_data = destination.get_internal_data_mut();
        let sliced_array_view: ArrayView3<f32> = source.get_internal_data().slice(
            s![crop_from.lower_left_row_major().0 .. crop_from.upper_right_row_major().0,
                crop_from.lower_left_row_major().1 .. crop_from.upper_right_row_major().1 , 0..number_color_channels]
        );
        destination_data = &mut sliced_array_view.into_owned();
        Ok(())
    }
    
    fn resize(&self, source: &ImageFrame, destination: &mut ImageFrame) -> Result<(), FeagiDataProcessingError> {
        // Uses Nearest Neighbor. Not pretty but fast
        let resize_xy_to = self.final_resize_xy_to.unwrap();
        let source_resolution = source.get_internal_resolution();
        let dest_resolution_f: (f32, f32) = (resize_xy_to.0 as f32, resize_xy_to.1 as f32);
        let source_resolution_f: (f32, f32) = (source_resolution.0 as f32, source_resolution.1 as f32);
        
        let source_data = source.get_internal_data();
        let mut destination_data = destination.get_internal_data_mut();
        
        for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
            let nearest_neighbor_coordinate_y: usize = (((y as f32) / dest_resolution_f.1) * source_resolution_f.1).floor() as usize;
            let nearest_neighbor_coordinate_x: usize = (((x as f32) / dest_resolution_f.0) * source_resolution_f.0).floor() as usize;
            let nearest_neighbor_channel_value: f32 = source_data[(
                nearest_neighbor_coordinate_x,
                nearest_neighbor_coordinate_y,
                c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(())
    }
    
    
    fn crop_and_resize(&self, source: &ImageFrame, destination: &mut ImageFrame) -> Result<(), FeagiDataProcessingError> {

        let crop_resolution: (usize, usize) = self.cropping_from.unwrap().enclosed_area_width_height();

        let resolution: (usize, usize) = self.final_resize_xy_to.unwrap();
        let resolution_f: (f32, f32) = (resolution.0 as f32, resolution.1 as f32);
        let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

        let dist_factor_yx: (f32, f32) = (
            crop_resolution_f.1 / resolution_f.1,
            crop_resolution_f.0 / resolution_f.0);

        let upper_left_corner_offset_yx: (usize, usize) = (
            self.cropping_from.unwrap().upper_left_row_major().0,
            self.cropping_from.unwrap().upper_left_row_major().1,
        );

        let source_data = source.get_internal_data();
        let mut destination_data = destination.get_internal_data_mut();

        for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
            let nearest_neighbor_coordinate_from_source_y: usize = ((y as f32) * dist_factor_yx.0).floor() as usize;
            let nearest_neighbor_coordinate_from_source_x: usize = ((x as f32) * dist_factor_yx.1).floor() as usize;
            let nnc_y_with_offset = nearest_neighbor_coordinate_from_source_y + upper_left_corner_offset_yx.0;
            let nnc_x_with_offset = nearest_neighbor_coordinate_from_source_x + upper_left_corner_offset_yx.1;

            let nearest_neighbor_channel_value: f32 = source_data[(
                nnc_y_with_offset,
                nnc_x_with_offset,
                c)];
            *color_val = nearest_neighbor_channel_value;
        };
        Ok(())
    }

    //endregion
    
    //region pixel plane processing
    
    fn apply_brightness_scale(&self, input_pixel_plane: f32, multiplier: f32) -> f32 {
        input_pixel_plane * multiplier
    }
    
    
    
    fn rg_to_r(&self,r: f32, g: f32) -> f32 {
        return r * g / 2.0;
    }
    
    fn rgb_to_r(&self,r: f32, g: f32, b: f32) -> f32 {
        return r * g * b / 3.0; // TODO: This is likely not correct!
    }
    
    
    //endregion


}