use ndarray::{s, Array3, ArrayView3};
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::image_descriptors::{ChannelLayout, ColorSpace, CornerPoints, ImageFrameProperties, MemoryOrderLayout};
use crate::io_data::ImageFrame;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImageFrameTransformerDefinition { // these properties are in order of how they are applied
    input_image_properties: ImageFrameProperties,
    cropping_from: Option<CornerPoints>, 
    final_resize_xy_to: Option<(usize, usize)>,
    convert_color_space_to: Option<ColorSpace>,
    multiply_brightness_by: Option<f32>,
    change_contrast_by: Option<f32>,
    convert_to_grayscale: bool, // Only allowed on RGB
}

impl std::fmt::Display for ImageFrameTransformerDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let initial = format!("Expecting {}.", self.input_image_properties);
        let mut steps: String = match (self.cropping_from, self.final_resize_xy_to) {
            (None, None) => format!("Keeping input size of <{}, {}> (no cropping from or resizing to)", self.input_image_properties.get_expected_xy_resolution().0, self.input_image_properties.get_expected_xy_resolution().1),
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
        write!(f, "ImageFrameCleanupDefinition({} {})", initial, steps)
    }
}

impl ImageFrameTransformerDefinition {
    
    pub fn new(input_image_properties: ImageFrameProperties) -> ImageFrameTransformerDefinition {
        ImageFrameTransformerDefinition {
            input_image_properties,
            cropping_from: None,
            final_resize_xy_to: None,
            multiply_brightness_by: None,
            change_contrast_by: None,
            convert_color_space_to: None,
            convert_to_grayscale: false,
        }
    }

    pub fn get_input_image_properties(&self) -> &ImageFrameProperties { &self.input_image_properties }
    
    pub fn get_output_image_properties(&self) -> ImageFrameProperties {
        let resolution = match (self.cropping_from, self.final_resize_xy_to) {
            (None, None) => self.input_image_properties.get_expected_xy_resolution(),
            (Some(cropping_from), None) => cropping_from.enclosed_area_width_height(),
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
    
    pub fn verify_input_image_allowed(&self, verifying_image: &ImageFrame) -> Result<(), FeagiDataProcessingError> {
        self.input_image_properties.verify_image_frame_matches_properties(verifying_image)
    }
    

    // TODO 2 / 4 channel pipelines!
    // Due to image segmentor, I would argue the most common route is crop + resize + grayscale

    pub fn process_image(&self, source: &ImageFrame, destination: &mut ImageFrame) -> Result<(), FeagiDataProcessingError> {
        match self {

            // If no fast path, use this slower universal one
            _ => {
                // This function is much slower, There may be some optimization work possible, but ensure the most common step combinations have an accelerated path
                let is_cropping_is_resizing = (self.cropping_from, self.final_resize_xy_to);

                let mut processing = source.clone();
                match is_cropping_is_resizing {
                    (None, None) => {
                        // don't do anything
                    }
                    (Some(cropping_from), None) => {
                        crop(source, &mut processing, &cropping_from, self.get_output_channel_count())?;
                    }
                    (None, Some(final_resize_xy_to)) => {
                        resize(source, &mut processing, &final_resize_xy_to)?;
                    }
                    (Some(cropping_from), Some(final_resize_xy_to)) => {
                        crop_and_resize(source, &mut processing, &cropping_from, &final_resize_xy_to)?;
                    }
                };

                match self.convert_color_space_to {
                    None => {
                        // Do Nothing
                    }
                    Some(color_space) => {
                        return Err(FeagiDataProcessingError::NotImplemented)
                    }
                }

                match self.multiply_brightness_by {
                    None => {
                        // Do Nothing
                    }
                    Some(brightness_multiplier) => {
                        processing.change_brightness(brightness_multiplier)?;
                    }
                }

                match self.change_contrast_by {
                    None => {
                        // Do Nothing
                    }
                    Some(contrast_multiplier) => {
                        processing.change_contrast(contrast_multiplier)?;
                    }
                }

                if self.convert_to_grayscale {
                    return Err(FeagiDataProcessingError::NotImplemented)
                }

                *destination = processing;
                Ok(())
            }

            // Do literally nothing, just copy the data
            ImageFrameTransformerDefinition {
                input_image_properties,
                cropping_from: None,
                final_resize_xy_to: None,
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                *destination = source.clone();
                Ok(())
            }

            // Only cropping
            ImageFrameTransformerDefinition {
                input_image_properties,
                cropping_from: Some(cropping_from),
                final_resize_xy_to: None,
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                crop(source, destination, cropping_from, self.get_output_channel_count())
            }

            // Only resizing
            ImageFrameTransformerDefinition {
                input_image_properties,
                cropping_from: None,
                final_resize_xy_to: Some(final_resize_xy_to),
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                resize(source, destination, final_resize_xy_to)
            }

            // Only grayscaling
            ImageFrameTransformerDefinition {
                input_image_properties,
                cropping_from: None,
                final_resize_xy_to: None,
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: true
            } => {
                to_grayscale(source, destination, self.input_image_properties.get_expected_color_space())
            }

            // Cropping, Resizing
            ImageFrameTransformerDefinition {
                input_image_properties,
                cropping_from: Some(cropping_from),
                final_resize_xy_to: Some(final_resize_xy_to),
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: false
            } => {
                crop_and_resize(source, destination, cropping_from, final_resize_xy_to)
            }

            // Cropping, Resizing, Grayscaling (the most common with segmentation vision)
            ImageFrameTransformerDefinition {
                input_image_properties,
                cropping_from: Some(cropping_from),
                final_resize_xy_to: Some(final_resize_xy_to),
                convert_color_space_to: None,
                multiply_brightness_by: None,
                change_contrast_by:None,
                convert_to_grayscale: true
            } => {
                crop_and_resize_and_grayscale(source, destination, cropping_from, final_resize_xy_to, self.input_image_properties.get_expected_color_space())
            }

        }


    }

    //region set settings
    // TODO create clear all, clear individual settings

    // TODO safety bound checks!

    pub fn set_cropping_from(&mut self, lower_left_xy_point_inclusive: (usize, usize), upper_right_xy_point_exclusive: (usize, usize)) -> Result<&Self, FeagiDataProcessingError> {
        let corner_points = CornerPoints::new_from_cartesian(lower_left_xy_point_inclusive, upper_right_xy_point_exclusive, self.input_image_properties.get_expected_xy_resolution())?;
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
        if self.input_image_properties.get_expected_color_channel_layout() == ChannelLayout::GrayScale {
            return Err(IODataError::InvalidParameters("Image is already Grayscale!".into()).into())
        }
        
        if self.input_image_properties.get_expected_color_channel_layout() == ChannelLayout::RG {
            return Err(FeagiDataProcessingError::NotImplemented)
        }
        self.convert_to_grayscale = true;
        Ok(self)
    }

    //endregion
    
    //region helpers

    fn get_output_channel_count(&self) -> usize {
        if self.convert_to_grayscale {
            return 1;
        }
        self.input_image_properties.get_expected_color_channel_layout().into()
    }
    

    //endregion
    

    
}

//region source destination processors

fn crop(source: &ImageFrame, destination: &mut ImageFrame, crop_from: &CornerPoints, number_output_color_channels: usize) -> Result<(), FeagiDataProcessingError> {
    let mut destination_data = destination.get_internal_data_mut();
    let sliced_array_view: ArrayView3<f32> = source.get_internal_data().slice(
        s![crop_from.lower_left_row_major().0 .. crop_from.upper_right_row_major().0,
                crop_from.lower_left_row_major().1 .. crop_from.upper_right_row_major().1 , 0..number_output_color_channels]
    );
    destination_data = &mut sliced_array_view.into_owned();
    Ok(())
}

fn resize(source: &ImageFrame, destination: &mut ImageFrame, resize_xy_to: &(usize, usize)) -> Result<(), FeagiDataProcessingError> {
    // Uses Nearest Neighbor. Not pretty but fast
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

fn to_grayscale(source: &ImageFrame, destination: &mut ImageFrame, output_color_space: ColorSpace) -> Result<(), FeagiDataProcessingError> {
    // NOTE: destination should be grayscale and source should be RGB or RGBA
    let source_data = source.get_internal_data();
    let mut destination_data = destination.get_internal_data_mut();
    let (r_scale, g_scale, b_scale) = match output_color_space {
        ColorSpace::Linear => {(0.2126f32, 0.7152f32, 0.072f32)} // Using formula from https://stackoverflow.com/questions/17615963/standard-rgb-to-grayscale-conversion
        ColorSpace::Gamma => {(0.299f32, 0.587f32, 0.114f32)}  // https://www.youtube.com/watch?v=uKeKuaJ4nlw (I forget)
    };
    // TODO look into premultiplied alpha handling!

    for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
        // TODO this is bad, we shouldnt be iterating over color channel and matching like this. Major target for optimization!
        if c != 0 { continue; }
        *color_val = r_scale * source_data[(y, x, 0)] + b_scale * source_data[(y, x, 1)] + g_scale * source_data[(y, x, 2)];
    }
    Ok(())
    

    
}

fn crop_and_resize(source: &ImageFrame, destination: &mut ImageFrame, crop_from: &CornerPoints, resize_xy_to: &(usize, usize)) -> Result<(), FeagiDataProcessingError> {

    let crop_resolution: (usize, usize) = crop_from.enclosed_area_width_height();
    let resolution_f: (f32, f32) = (resize_xy_to.0 as f32, resize_xy_to.1 as f32);
    let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

    let dist_factor_yx: (f32, f32) = (
        crop_resolution_f.1 / resolution_f.1,
        crop_resolution_f.0 / resolution_f.0);

    let upper_left_corner_offset_yx: (usize, usize) = (
        crop_from.upper_left_row_major().0,
        crop_from.upper_left_row_major().1,
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

fn crop_and_resize_and_grayscale(source: &ImageFrame, destination: &mut ImageFrame, crop_from: &CornerPoints, resize_xy_to: &(usize, usize), output_color_space: ColorSpace) -> Result<(), FeagiDataProcessingError> {

    let crop_resolution: (usize, usize) = crop_from.enclosed_area_width_height();
    let resolution_f: (f32, f32) = (resize_xy_to.0 as f32, resize_xy_to.1 as f32);
    let crop_resolution_f: (f32, f32) = (crop_resolution.0 as f32, crop_resolution.1 as f32);

    let dist_factor_yx: (f32, f32) = (
        crop_resolution_f.1 / resolution_f.1,
        crop_resolution_f.0 / resolution_f.0);

    let upper_left_corner_offset_yx: (usize, usize) = (
        crop_from.upper_left_row_major().0,
        crop_from.upper_left_row_major().1,
    );

    let source_data = source.get_internal_data();
    let mut destination_data = destination.get_internal_data_mut();
    let (r_scale, g_scale, b_scale) = match output_color_space {
        ColorSpace::Linear => {(0.2126f32, 0.7152f32, 0.072f32)} // Using formula from https://stackoverflow.com/questions/17615963/standard-rgb-to-grayscale-conversion
        ColorSpace::Gamma => {(0.299f32, 0.587f32, 0.114f32)}
    };
    // TODO look into premultiplied alpha handling!

    for ((y,x,c), color_val) in destination_data.indexed_iter_mut() {
        // TODO this is bad, we shouldnt be iterating over color channel and matching like this. Major target for optimization!
        if c != 0 { continue; }
        let nearest_neighbor_coordinate_from_source_y: usize = ((y as f32) * dist_factor_yx.0).floor() as usize;
        let nearest_neighbor_coordinate_from_source_x: usize = ((x as f32) * dist_factor_yx.1).floor() as usize;
        let nnc_y_with_offset = nearest_neighbor_coordinate_from_source_y + upper_left_corner_offset_yx.0;
        let nnc_x_with_offset = nearest_neighbor_coordinate_from_source_x + upper_left_corner_offset_yx.1;
        let nearest_neighbor_channel_r: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            0)];
        let nearest_neighbor_channel_g: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            1)];
        let nearest_neighbor_channel_b: f32 = source_data[(
            nnc_y_with_offset,
            nnc_x_with_offset,
            2)];
        
        *color_val = r_scale * nearest_neighbor_channel_r + b_scale * nearest_neighbor_channel_g + g_scale * nearest_neighbor_channel_b;
    }
    Ok(())
    
}
//endregion
