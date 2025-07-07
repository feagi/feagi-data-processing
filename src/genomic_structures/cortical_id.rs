use crate::error::{FeagiDataProcessingError, GenomeError};
use super::{CorticalType, SensorCorticalType, MotorCorticalType, CoreCorticalType};
use super::index_types::CorticalGroupingIndex;


use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorticalID {
    pub(crate) bytes: [u8; CorticalID::CORTICAL_ID_LENGTH],
}

impl fmt::Display for CorticalID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = safe_bytes_to_string(&self.bytes);
        write!(f, "{}", ch)
    }
}

impl CorticalID {
    pub const CORTICAL_ID_LENGTH: usize = 6;
    
    pub fn new_custom_cortical_area_id(desired_id_string: String) -> Result<Self, FeagiDataProcessingError> {
        Self::verify_input_length(&desired_id_string)?;
        Self::verify_input_ascii(&desired_id_string)?;
        Self::verify_allowed_characters(&desired_id_string)?;
        
        let bytes = desired_id_string.as_bytes();
        let bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
        if bytes[0] != b'c' {
            return Err(GenomeError::InvalidCorticalID(format!("A custom cortical area ID must start with 'c'! Cortical area given: {}", desired_id_string)).into());
        }
        Ok(CorticalID { bytes: *bytes })
    }

    pub fn new_memory_cortical_area_id(desired_id_string: String) -> Result<Self, FeagiDataProcessingError> {

        Self::verify_input_length(&desired_id_string)?;
        Self::verify_input_ascii(&desired_id_string)?;
        Self::verify_allowed_characters(&desired_id_string)?;
        
        let bytes = desired_id_string.as_bytes();
        let bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH] = bytes.try_into().unwrap();
        if bytes[0] != b'm' {
            return Err(GenomeError::InvalidCorticalID(format!("A memory cortical area ID must start with 'm'! Cortical area given: {}", desired_id_string)).into());
        }
        Ok(CorticalID { bytes: *bytes })
    }

    pub fn new_core_cortical_area_id(core_type: CoreCorticalType) -> Result<Self, GenomeError> {
        Ok(core_type.to_cortical_id())
    }

    pub fn new_sensor_cortical_area_id(input_type: SensorCorticalType, input_index: CorticalGroupingIndex) -> Result<Self, GenomeError> {
        Ok(input_type.to_cortical_id(input_index))
    }

    pub fn new_motor_cortical_area_id(output_type: MotorCorticalType, output_index: CorticalGroupingIndex) -> Result<Self, GenomeError> {
        Ok(output_type.to_cortical_id(output_index))
    }

    pub fn create_ordered_cortical_areas_for_segmented_vision(camera_index: CorticalGroupingIndex, is_grayscale: bool) -> [CorticalID; 9] {
        if is_grayscale {
            [
                SensorCorticalType::VisionCenterGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionBottomLeftGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionMiddleLeftGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionTopLeftGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionTopMiddleGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionTopRightGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionMiddleRightGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionBottomRightGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionBottomMiddleGray.to_cortical_id(camera_index),
            ]
        }
        else {
            [
                SensorCorticalType::VisionCenterColor.to_cortical_id(camera_index),
                SensorCorticalType::VisionBottomLeftGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionMiddleLeftGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionTopLeftGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionTopMiddleGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionTopRightGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionMiddleRightGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionBottomRightGray.to_cortical_id(camera_index),
                SensorCorticalType::VisionBottomMiddleGray.to_cortical_id(camera_index),
            ]
        }
    }
    pub fn from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<Self, FeagiDataProcessingError> {
        let as_string = String::from_utf8(bytes.to_vec());
        if as_string.is_err() {
            return Err(GenomeError::InvalidCorticalID("Unable to parse cortical ID as ASCII!".into()).into());
        }
        
        let as_string = as_string.unwrap();
        Self::verify_input_ascii(&as_string)?;
        Self::verify_allowed_characters(&as_string)?;
        
        let _ = CorticalType::get_type_from_bytes(bytes)?; // if type is invalid, error
        Ok(CorticalID {bytes: *bytes})
    }

    pub fn from_string(string: String) -> Result<Self, FeagiDataProcessingError> {
        
        Self::verify_input_length(&string)?;
        Self::verify_input_ascii(&string)?;
        Self::verify_allowed_characters(&string)?;
        
        let bytes: [u8; CorticalID::CORTICAL_ID_LENGTH] = string.as_bytes().try_into().unwrap();
        let _ = CorticalType::get_type_from_bytes(&bytes)?; // if type is invalid, error
        Ok(CorticalID {bytes })
    }
    

    
    pub fn as_bytes(&self) -> &[u8; CorticalID::CORTICAL_ID_LENGTH] {
        &self.bytes
    }

    pub fn write_bytes_at(&self, target: &mut [u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<(), FeagiDataProcessingError> {
        target.copy_from_slice(&self.bytes);
        Ok(())
    }

    pub fn to_identifier_ascii_string(&self) -> String {
        safe_bytes_to_string(&self.bytes)
    }
    
    pub fn get_cortical_type(&self) -> CorticalType {
        CorticalType::get_type_from_bytes(&self.bytes).unwrap() // will never error
    }
    
    fn verify_input_length(string: &String) -> Result<(), GenomeError> {
        if string.len() != CorticalID::CORTICAL_ID_LENGTH {
            return Err(GenomeError::InvalidCorticalID(format!("A cortical ID must have a length of {}! Given cortical ID '{}' is not!", CorticalID::CORTICAL_ID_LENGTH, string)).into());
        }
        Ok(())
    }

    fn verify_input_ascii(string: &String) -> Result<(), GenomeError> {
        if !string.is_ascii() {
            return Err(GenomeError::InvalidCorticalID(format!("A cortical ID must be entirely ASCII! Given cortical ID '{}' is not!", string)).into());
        }
        Ok(())
    }

    fn verify_allowed_characters(string: &String) -> Result<(), GenomeError> {
        if !string.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(GenomeError::InvalidCorticalID(format!("A cortical ID must be made only of alphanumeric characters and underscores! Given cortical ID '{}' is not!", string)).into());
        }
        Ok(())
    }
    
    
}


// This function assumes that we know the bytes are valid ASCII
fn safe_bytes_to_string(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

