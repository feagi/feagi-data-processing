//! Core I/O data type definitions and conversions for FEAGI.
//!
//! This module defines the fundamental data types used throughout the FEAGI system
//! for representing different kinds of input and output data. It provides both
//! type identifiers (IOTypeVariant) and typed data containers (IOTypeData) to 
//! allow functions to pass various data types while keeping Rust's type system happy

use std::cmp::PartialEq;
use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::{ImageFrame, SegmentedImageFrame};
use crate::io_data::image::descriptors::ImageFrameProperties;
//region IOTypeVariant

/// Type identifiers for all supported I/O data types in FEAGI.
///
/// `IOTypeVariant` provides type identification without carrying the actual data values.
/// This is useful for type checking, stream processor compatibility validation,
/// and system configuration without needing to handle the actual data.
///
/// # Supported Types
///
/// ## Numeric Types
/// - **F32**: Standard 32-bit floating point values (any finite value that isn't NaN)
/// - **F32Normalized0To1**: Normalized floats in range [0.0, 1.0] for positive signals
/// - **F32NormalizedM1To1**: Normalized floats in range [-1.0, 1.0] for bidirectional signals
///
/// ## Visual Types
/// - **ImageFrame**: Single image/frame data for vision processing
/// - **SegmentedImageFrame**: Multi-segment vision frames for peripheral vision
///
/// # Example
///
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::{IOTypeVariant, IOTypeData};
///
/// // Type checking without data
/// let expected_type = IOTypeVariant::F32Normalized0To1;
/// let sensor_data = IOTypeData::new_0_1_f32(0.75).unwrap();
/// 
/// assert!(expected_type.is_of(&sensor_data));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOTypeVariant {
    F32,
    F32Normalized0To1,
    F32NormalizedM1To1,
    ImageFrame(Option<ImageFrameProperties>),
    SegmentedImageFrame(Option<[ImageFrameProperties; 9]>),
}

impl IOTypeVariant {
    /// Checks if the given IOTypeData matches this type variant.
    ///
    /// This method tests whether a specific IOTypeData instance is of the type
    /// represented by this variant, enabling type validation without extracting values.
    ///
    /// # Arguments
    /// * `io_type` - The IOTypeData instance to check
    ///
    /// # Returns
    /// `true` if the data type matches this variant, `false` otherwise
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::{IOTypeVariant, IOTypeData};
    ///
    /// let data = IOTypeData::new_f32(42.0).unwrap();
    /// assert!(IOTypeVariant::F32.is_of(&data));
    /// assert!(!IOTypeVariant::F32Normalized0To1.is_of(&data));
    /// ```
    pub fn is_of(&self, io_type: &IOTypeData) -> bool { 
        IOTypeVariant::from(io_type) == *self 
    }
}

impl std::fmt::Display for IOTypeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            IOTypeVariant::F32 => write!(f, "IOTypeVariant(F32)"),
            IOTypeVariant::F32Normalized0To1 => write!(f, "IOTypeVariant(F32 [Normalized 0<->1])"),
            IOTypeVariant::F32NormalizedM1To1 => write!(f, "IOTypeVariant(F32 [Normalized -1<->1])"),
            IOTypeVariant::ImageFrame(image_properties) => {
                let s: String = match image_properties {
                    Some(properties) => format!("ImageFrame({})", properties.to_string()),
                    None => "ImageFrame(No Requirements)".to_string(),
                };
                write!(f, "{}", s)
            } 
            IOTypeVariant::SegmentedImageFrame(segment_properties) => {
                let s: String = match segment_properties {
                    None => "No Requirements".to_string(),
                    Some(properties) => {
                        format!("[Lower Left: {}, Lower Middle: {}, Lower Right: {}, Middle Left: {}, Middle Middle: {}, Middle Right: {}, Upper Left: {}, Upper Middle: {}, Upper Right: {}]", 
                                properties[0].to_string(), properties[1].to_string(), properties[2].to_string(), properties[3].to_string(), properties[4].to_string(), properties[5].to_string(), properties[6].to_string(), properties[7].to_string(), properties[8].to_string())
                    }
                };
                write!(f, "SegmentedImageFrame({})", s)
            }
        }
    }
}

impl From<IOTypeData> for IOTypeVariant {
    fn from(io_type: IOTypeData) -> Self {
        match io_type { 
            IOTypeData::F32(_) => IOTypeVariant::F32,
            IOTypeData::F32Normalized0To1(_) => IOTypeVariant::F32Normalized0To1,
            IOTypeData::F32NormalizedM1To1(_) => IOTypeVariant::F32NormalizedM1To1,
            IOTypeData::ImageFrame(image) => IOTypeVariant::ImageFrame(Some(image.get_image_frame_properties())),
            IOTypeData::SegmentedImageFrame(segments) => IOTypeVariant::SegmentedImageFrame(Some(segments.get_image_frame_properties())),
        }
    }
}

impl From<&IOTypeData> for IOTypeVariant {
    fn from(io_type: &IOTypeData) -> Self {
        match io_type {
            IOTypeData::F32(_) => IOTypeVariant::F32,
            IOTypeData::F32Normalized0To1(_) => IOTypeVariant::F32Normalized0To1,
            IOTypeData::F32NormalizedM1To1(_) => IOTypeVariant::F32NormalizedM1To1,
            IOTypeData::ImageFrame(image) => IOTypeVariant::ImageFrame(Some(image.get_image_frame_properties())),
            IOTypeData::SegmentedImageFrame(segments) => IOTypeVariant::SegmentedImageFrame(Some(segments.get_image_frame_properties())),
        }
    }
}

//endregion

//region IOTypeData

/// Unified container for all supported I/O data types with values.
///
/// `IOTypeData` is the primary data container used throughout the FEAGI system
/// for representing typed input and output data. Each variant contains both the
/// type information and the actual data value, providing type safety and validation.
///
/// # Data Types and Validation
///
/// ## Numeric Types
/// All numeric types validate that values are finite (not NaN or infinite):
///
/// - **F32(f32)**: Any finite 32-bit float value
/// - **F32Normalized0To1(f32)**: Values in range [0.0, 1.0] for positive signals like brightness
/// - **F32NormalizedM1To1(f32)**: Values in range [-1.0, 1.0] for bidirectional signals like motor control
///
/// ## Visual Types
/// - **ImageFrame(ImageFrame)**: Single image/frame data with metadata and processing capabilities
/// - **SegmentedImageFrame(SegmentedImageFrame)**: Multi-segment vision frames for peripheral vision
///
/// # Construction and Validation
///
/// Use the validated constructors to ensure data integrity:
/// - `new_f32()` for general floating point values
/// - `new_0_1_f32()` for normalized positive values
/// - `new_m1_1_f32()` for normalized bidirectional values
/// - `From` traits for image types
///
/// # Type Conversion
///
/// The enum supports safe conversion using `TryFrom` traits that preserve type information
/// and provide clear error messages for incompatible conversions.
///
/// # Usage in Processing Pipelines
///
/// Stream cache processors receive and emit IOTypeData, allowing for type-safe
/// data transformation pipelines with runtime type checking.
///
/// # Example
///
/// ```rust
/// use feagi_core_data_structures_and_processing::io_data::IOTypeData;
///
/// // Create validated data
/// let temperature = IOTypeData::new_f32(23.5).unwrap();
/// let brightness = IOTypeData::new_0_1_f32(0.8).unwrap();
/// let motor_speed = IOTypeData::new_m1_1_f32(-0.5).unwrap();
///
/// // Type-safe extraction
/// let temp_value: f32 = temperature.try_into().unwrap();
/// assert_eq!(temp_value, 23.5);
/// ```
#[derive(Debug, Clone)]
pub enum IOTypeData
{
    F32(f32),
    F32Normalized0To1(f32),
    F32NormalizedM1To1(f32),
    ImageFrame(ImageFrame),
    SegmentedImageFrame(SegmentedImageFrame),
}

impl std::fmt::Display for IOTypeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            IOTypeData::F32(float) => write!(f, "IOTypeData(f32({}))", float),
            IOTypeData::F32Normalized0To1(float) => write!(f, "IOTypeData(f32[Normalized 0<->1]({}))", float),
            IOTypeData::F32NormalizedM1To1(float) => write!(f, "IOTypeData(f32[Normalized -1<->1]({}))", float),
            IOTypeData::ImageFrame(frame) => write!(f, "IOTypeData({})", frame),
            IOTypeData::SegmentedImageFrame(frame) => write!(f, "IOTypeData({})", frame),
        }
    }
}

// NOTE: Not implementing "From<f32> for IOTypeData" since there are multiple paths

impl From<ImageFrame> for IOTypeData {
    fn from(value: ImageFrame) -> Self {
        IOTypeData::ImageFrame(value)
    }
}

impl From<SegmentedImageFrame> for IOTypeData {
    fn from(value: SegmentedImageFrame) -> Self {
        IOTypeData::SegmentedImageFrame(value)
    }
}

impl TryFrom<IOTypeData> for f32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value { 
            IOTypeData::F32(float) => Ok(float),
            IOTypeData::F32Normalized0To1(float) => Ok(float),
            IOTypeData::F32NormalizedM1To1(float) => Ok(float),
            _ => Err(IODataError::InvalidParameters("This variable is not a f32 type value!".into()).into()),
        }
    }
}

impl TryFrom<&IOTypeData> for f32 {
    type Error = FeagiDataProcessingError;
    fn try_from(value: &IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::F32(float) => Ok(*float),
            IOTypeData::F32Normalized0To1(float) => Ok(*float),
            IOTypeData::F32NormalizedM1To1(float) => Ok(*float),
            _ => Err(IODataError::InvalidParameters("This variable is not a f32 type value!".into()).into()),
        }
    }
}


impl TryFrom<IOTypeData> for ImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image) => Ok(image),
            _ => Err(IODataError::InvalidParameters("This variable is not a Image Frame!".into()).into()),
        }
    }
}

impl<'a> TryFrom<&'a IOTypeData> for &'a ImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: &'a IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a Image Frame!".into()).into()),
        }
    }
}

impl<'a> TryFrom<&'a mut IOTypeData> for &'a mut ImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: &'a mut IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::ImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a Image Frame!".into()).into()),
        }
    }
}

impl TryFrom<IOTypeData> for SegmentedImageFrame {
    type Error = FeagiDataProcessingError;
    
    fn try_from(value: IOTypeData) -> Result<Self, Self::Error> {
        match value { 
            IOTypeData::SegmentedImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a SegmentedImageFrame!".into()).into()),
        }
    }
}

impl<'a> TryFrom<&'a IOTypeData> for &'a SegmentedImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: &'a IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::SegmentedImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a Segmented Image Frame!".into()).into()),
        }
    }
}

impl<'a> TryFrom<&'a mut IOTypeData> for &'a mut SegmentedImageFrame {
    type Error = FeagiDataProcessingError;

    fn try_from(value: &'a mut IOTypeData) -> Result<Self, Self::Error> {
        match value {
            IOTypeData::SegmentedImageFrame(image_ref) => Ok(image_ref),
            _ => Err(IODataError::InvalidParameters("This variable is not a Segmented Image Frame!".into()).into()),
        }
    }
}



impl IOTypeData {
    /// Creates a new F32 IOTypeData with validation.
    ///
    /// Validates that the input value is finite (not NaN or infinite) before creating
    /// the data container. This ensures data integrity throughout the processing pipeline.
    ///
    /// # Arguments
    /// * `value` - The f32 value to store (must be finite)
    ///
    /// # Returns
    /// * `Ok(IOTypeData::F32)` - Successfully created data container
    /// * `Err(FeagiDataProcessingError)` - If value is NaN or infinite
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::IOTypeData;
    ///
    /// let valid_data = IOTypeData::new_f32(42.5).unwrap();
    /// let invalid_data = IOTypeData::new_f32(f32::NAN);
    /// assert!(invalid_data.is_err());
    /// ```
    pub fn new_f32(value: f32) -> Result<Self, FeagiDataProcessingError> {
        if value.is_nan() || value.is_infinite() {
            return Err(IODataError::InvalidParameters("Input value cannot be NaN or Infinite!".into()).into());
        }
        Ok(Self::F32(value))
    }
    
    /// Creates a new normalized F32 IOTypeData in range [0.0, 1.0].
    ///
    /// Validates that the input value is finite and within the normalized range [0.0, 1.0].
    /// This type is commonly used for positive signals like brightness, intensity, or
    /// probability values.
    ///
    /// # Arguments
    /// * `value` - The f32 value to store (must be in range [0.0, 1.0])
    ///
    /// # Returns
    /// * `Ok(IOTypeData::F32Normalized0To1)` - Successfully created normalized data
    /// * `Err(FeagiDataProcessingError)` - If value is invalid or out of range
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::IOTypeData;
    ///
    /// let brightness = IOTypeData::new_0_1_f32(0.75).unwrap();
    /// let invalid = IOTypeData::new_0_1_f32(1.5);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new_0_1_f32(value: f32) -> Result<Self, FeagiDataProcessingError> {
        if value.is_nan() || value.is_infinite() {
            return Err(IODataError::InvalidParameters("Input value cannot be NaN or Infinite!".into()).into());
        }
        if value < 0.0 || value > 1.0 {
            return Err(IODataError::InvalidParameters("Input value must be between 0 and 1!".into()).into());
        }
        Ok(Self::F32Normalized0To1(value))
    }

    /// Creates a new normalized F32 IOTypeData in range [-1.0, 1.0].
    ///
    /// Validates that the input value is finite and within the normalized range [-1.0, 1.0].
    /// This type is commonly used for bidirectional signals like motor control, steering,
    /// or any value that needs to represent both positive and negative directions.
    ///
    /// # Arguments
    /// * `value` - The f32 value to store (must be in range [-1.0, 1.0])
    ///
    /// # Returns
    /// * `Ok(IOTypeData::F32NormalizedM1To1)` - Successfully created normalized data
    /// * `Err(FeagiDataProcessingError)` - If value is invalid or out of range
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::io_data::IOTypeData;
    ///
    /// let motor_speed = IOTypeData::new_m1_1_f32(-0.5).unwrap(); // 50% reverse
    /// let steering = IOTypeData::new_m1_1_f32(0.3).unwrap();     // 30% right
    /// let invalid = IOTypeData::new_m1_1_f32(2.0);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new_m1_1_f32(value: f32) -> Result<Self, FeagiDataProcessingError> {
        if value.is_nan() || value.is_infinite() {
            return Err(IODataError::InvalidParameters("Input value cannot be NaN or Infinite!".into()).into());
        }
        if value < -1.0 || value > 1.0 {
            return Err(IODataError::InvalidParameters("Input value must be between -1 and 1!".into()).into());
        }
        Ok(Self::F32NormalizedM1To1(value))
    }
}

//endregion
