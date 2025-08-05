//! Cortical area identifiers for FEAGI genomic structures.
//!
//! This module provides the `CorticalID` type, which represents unique identifiers
//! for cortical areas within the FEAGI neural system. These identifiers follow a
//! strict 6-character ASCII format that encodes both the type and instance information
//! of cortical areas.

use crate::error::{FeagiDataProcessingError, GenomeError};
use super::{CorticalType, SensorCorticalType, MotorCorticalType, CoreCorticalType};
use super::index_types::CorticalGroupingIndex;


use std::fmt;

/// Unique identifier for cortical areas in the FEAGI neural system.
///
/// `CorticalID` provides a standardized way to identify and reference cortical areas
/// within a FEAGI genome. Each identifier is exactly 6 ASCII characters and encodes
/// both the type of cortical area and its instance index.
///
/// # Format Structure
///
/// The 6-character format varies by cortical area type:
///
/// ## Sensor Areas (Input)
/// Format: `i[type][index]` where:
/// - First character: `i` (input prefix)
/// - Characters 2-4: 3-character type code
/// - Characters 5-6: 2-character hexadecimal index (00-FF)
/// 
/// Example: `ivis00` = Vision sensor, index 0
///
/// ## Motor Areas (Output)  
/// Format: `o[type][index]` where:
/// - First character: `o` (output prefix)
/// - Characters 2-4: 3-character type code
/// - Characters 5-6: 2-character hexadecimal index (00-FF)
///
/// Example: `omot01` = Motor output, index 1
///
/// ## Core Areas
/// Format: `_[name]` where:
/// - First character: `_` (core prefix)
/// - Characters 2-6: 5-character fixed name
///
/// Examples: `_death`, `_power`
///
/// ## Custom Areas
/// Format: `c[name]` where:
/// - First character: `c` (custom prefix)  
/// - Characters 2-6: 5-character user-defined name
///
/// Example: `custom` (a full custom area name)
///
/// ## Memory Areas
/// Format: `m[name]` where:
/// - First character: `m` (memory prefix)
/// - Characters 2-6: 5-character user-defined name
///
/// Example: `memory` (a full memory area name)
///
/// # Character Constraints
/// - Exactly 6 ASCII characters
/// - Only alphanumeric characters and underscores allowed
/// - Case-sensitive identifiers
/// - No null bytes or non-ASCII characters
///
/// # Usage Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::genomic_structures::*;
///
/// // Create sensor cortical area ID
/// let vision_id = CorticalID::new_sensor_cortical_area_id(
///     SensorCorticalType::VisionCenterColor,
///     CorticalGroupingIndex::from(0)
/// ).unwrap();
/// assert_eq!(vision_id.to_identifier_ascii_string(), "iVcc00");
///
/// // Create from string
/// let custom_id = CorticalID::from_string("custom".to_string()).unwrap();
/// 
/// // Get the cortical type
/// let cortical_type = vision_id.get_cortical_type();
/// ```
///
/// # Thread Safety
/// `CorticalID` is `Copy` and completely thread-safe for concurrent access.
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
    /// The fixed length of all cortical identifiers in bytes.
    pub const CORTICAL_ID_LENGTH: usize = 6;
    
    /// Creates a new custom cortical area identifier.
    ///
    /// Custom cortical areas are user-defined processing areas that start with 'c'.
    /// They allow for flexible neural processing components beyond the standard
    /// sensor, motor, and core types.
    ///
    /// # Arguments
    /// * `desired_id_string` - The 6-character identifier (must start with 'c')
    ///
    /// # Returns
    /// * `Ok(CorticalID)` - Valid custom cortical area identifier
    /// * `Err(FeagiDataProcessingError)` - If validation fails
    ///
    /// # Validation Rules
    /// - Must be exactly 6 ASCII characters
    /// - Must start with 'c' (custom prefix)
    /// - Only alphanumeric characters and underscores allowed
    /// - No null bytes or non-ASCII characters
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalID;
    /// let custom_id = CorticalID::new_custom_cortical_area_id("custom".to_string()).unwrap();
    /// assert_eq!(custom_id.to_identifier_ascii_string(), "custom");
    /// ```
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

    /// Creates a new memory cortical area identifier.
    ///
    /// Memory cortical areas are persistent storage areas that start with 'm'.
    /// They provide long-term storage and retrieval capabilities for the neural
    /// system, maintaining information across processing cycles.
    ///
    /// # Arguments
    /// * `desired_id_string` - The 6-character identifier (must start with 'm')
    ///
    /// # Returns
    /// * `Ok(CorticalID)` - Valid memory cortical area identifier
    /// * `Err(FeagiDataProcessingError)` - If validation fails
    ///
    /// # Validation Rules
    /// - Must be exactly 6 ASCII characters
    /// - Must start with 'm' (memory prefix)
    /// - Only alphanumeric characters and underscores allowed
    /// - No null bytes or non-ASCII characters
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalID;
    /// let memory_id = CorticalID::new_memory_cortical_area_id("memory".to_string()).unwrap();
    /// assert_eq!(memory_id.to_identifier_ascii_string(), "memory");
    /// ```
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

    /// Creates a core cortical area identifier from a core type.
    ///
    /// Core cortical areas are essential system components with predefined identifiers.
    /// They handle fundamental system operations like power management and termination.
    ///
    /// # Arguments
    /// * `core_type` - The specific core cortical area type
    ///
    /// # Returns
    /// * `Ok(CorticalID)` - The corresponding core cortical area identifier
    /// * `Err(GenomeError)` - If core type conversion fails (unlikely)
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CoreCorticalType, CorticalID};
    /// let death_id = CorticalID::new_core_cortical_area_id(CoreCorticalType::Death).unwrap();
    /// assert_eq!(death_id.to_identifier_ascii_string(), "_death");
    /// ```
    pub fn new_core_cortical_area_id(core_type: CoreCorticalType) -> Result<Self, GenomeError> {
        Ok(core_type.to_cortical_id())
    }

    /// Creates a sensor cortical area identifier from a sensor type and index.
    ///
    /// Sensor cortical areas handle input processing from various sources like vision,
    /// audio, and other sensory data. The index allows for multiple instances of the
    /// same sensor type (e.g., multiple cameras).
    ///
    /// # Arguments
    /// * `input_type` - The specific sensor cortical area type
    /// * `input_index` - The grouping index for this sensor instance (0-255)
    ///
    /// # Returns
    /// * `Ok(CorticalID)` - The corresponding sensor cortical area identifier
    /// * `Err(GenomeError)` - If sensor type conversion fails (unlikely)
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalID, SensorCorticalType};
    /// let vision_id = CorticalID::new_sensor_cortical_area_id(
    ///     SensorCorticalType::VisionCenterColor,
    ///     CorticalGroupingIndex::from(0)
    /// ).unwrap();
    /// // Results in something like "ivisc0"
    /// ```
    pub fn new_sensor_cortical_area_id(input_type: SensorCorticalType, input_index: CorticalGroupingIndex) -> Result<Self, GenomeError> {
        Ok(input_type.to_cortical_id(input_index))
    }

    /// Creates a motor cortical area identifier from a motor type and index.
    ///
    /// Motor cortical areas handle output control to various actuators like motors,
    /// servos, and other control devices. The index allows for multiple instances of
    /// the same motor type (e.g., multiple motor controllers).
    ///
    /// # Arguments
    /// * `output_type` - The specific motor cortical area type
    /// * `output_index` - The grouping index for this motor instance (0-255)
    ///
    /// # Returns
    /// * `Ok(CorticalID)` - The corresponding motor cortical area identifier
    /// * `Err(GenomeError)` - If motor type conversion fails (unlikely)
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalID, MotorCorticalType};
    /// let motor_id = CorticalID::new_motor_cortical_area_id(
    ///     MotorCorticalType::RotoryMotor,
    ///     CorticalGroupingIndex::from(1)
    /// ).unwrap();
    /// // Results in something like "omot01"
    /// ```
    pub fn new_motor_cortical_area_id(output_type: MotorCorticalType, output_index: CorticalGroupingIndex) -> Result<Self, GenomeError> {
        Ok(output_type.to_cortical_id(output_index))
    }

    /// Creates a predefined set of cortical areas for segmented vision processing.
    ///
    /// This utility method generates 9 cortical areas arranged in a 3x3 grid pattern
    /// for processing segmented vision data. Each segment processes a different region
    /// of the visual field, allowing for spatial attention and region-specific processing.
    ///
    /// # Arguments
    /// * `camera_index` - The grouping index for this camera system (0-255)
    /// * `is_grayscale` - Whether to create grayscale (true) or color (false) vision areas
    ///
    /// # Returns
    /// Array of 9 CorticalID values arranged as:
    /// ```text
    /// [0] Center       [1] Bottom-Left   [2] Middle-Left
    /// [3] Top-Left     [4] Top-Middle    [5] Top-Right
    /// [6] Middle-Right [7] Bottom-Right  [8] Bottom-Middle
    /// ```
    ///
    /// # Vision Segmentation
    /// - **Center**: Primary focus area for detailed processing
    /// - **Surrounding segments**: Peripheral vision areas for context and motion detection
    /// - **Grayscale vs Color**: Determines whether segments process intensity or full color
    ///
    /// # Example
    /// ```rust
    /// // Create color vision segments for camera 0
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalGroupingIndex, CorticalID};
    /// let color_segments = CorticalID::create_ordered_cortical_areas_for_segmented_vision(
    ///     CorticalGroupingIndex::from(0), 
    ///     false
    /// );
    ///
    /// // Create grayscale vision segments for camera 1
    /// let gray_segments = CorticalID::create_ordered_cortical_areas_for_segmented_vision(
    ///     CorticalGroupingIndex::from(1), 
    ///     true
    /// );
    /// ```
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
    
    pub fn try_from_cortical_type(cortical_type: &CorticalType, io_cortical_index: CorticalGroupingIndex) -> Result<Self, FeagiDataProcessingError> {
        CorticalType::to_cortical_id(cortical_type, io_cortical_index)
    }
    
    
    /// Returns the raw bytes of this cortical identifier.
    ///
    /// Provides direct access to the underlying 6-byte array representation
    /// of the cortical ID. Useful for serialization, hashing, or low-level operations.
    ///
    /// # Returns
    /// Reference to the 6-byte array containing the ASCII characters
    pub fn as_bytes(&self) -> &[u8; CorticalID::CORTICAL_ID_LENGTH] {
        &self.bytes
    }

    /// Writes the cortical ID bytes to a target array.
    ///
    /// Copies the 6 bytes of this cortical ID into the provided target array.
    /// This is useful when you need to embed the cortical ID into a larger
    /// data structure or buffer.
    ///
    /// # Arguments
    /// * `target` - Mutable reference to a 6-byte array to write to
    ///
    /// # Returns
    /// * `Ok(())` - Bytes successfully copied
    pub(crate) fn write_bytes_at(&self, target: &mut [u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<(), FeagiDataProcessingError> {
        target.copy_from_slice(&self.bytes);
        Ok(())
    }

    /// Returns the cortical identifier as a readable ASCII string.
    ///
    /// Converts the internal byte representation back to a 6-character ASCII string.
    /// This is the primary method for getting a human-readable representation of
    /// the cortical ID.
    ///
    /// # Returns
    /// 6-character ASCII string representation of the cortical ID
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CoreCorticalType, CorticalID};
    /// let cortical_id = CorticalID::new_core_cortical_area_id(CoreCorticalType::Death).unwrap();
    /// let id_string = cortical_id.to_identifier_ascii_string();
    /// println!("Cortical Area: {}", id_string); // "_death"
    /// ```
    pub fn to_identifier_ascii_string(&self) -> String {
        safe_bytes_to_string(&self.bytes)
    }
    
    /// Returns the cortical type classification for this identifier.
    ///
    /// Analyzes the cortical ID format and returns the corresponding cortical type
    /// (Core, Sensory, Motor, Custom, or Memory). This provides access to type-specific
    /// properties and capabilities.
    ///
    /// # Returns
    /// The `CorticalType` classification for this cortical area
    ///
    /// # Panics
    /// This method will never panic as the cortical ID is validated during creation
    /// to ensure it always corresponds to a valid cortical type.
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

