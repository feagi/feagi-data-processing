//! Cortical area type classification and properties.
//!
//! This module defines the type system for FEAGI cortical areas, providing classification,
//! properties, and capabilities for different kinds of neural processing areas. The system
//! supports five main categories of cortical areas, each with specific characteristics
//! and constraints.

use std::fmt;
use crate::error::{FeagiBytesError, FeagiDataProcessingError, GenomeError, IODataError};
use crate::genomic_structures::cortical_id::{CorticalID};
use crate::genomic_structures::{SingleChannelDimensionRange};
use crate::genomic_structures::index_types::CorticalGroupingIndex;
use crate::neuron_data::xyzp::NeuronCoderVariantType;
use crate::sensor_definition;

macro_rules! define_io_cortical_types {
    (
        $cortical_io_type_enum_name:ident {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    default_coder_type: $default_coder_type:expr,
                }
            ),* $(,)?
        }
    ) => {

        // Type Enum
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $cortical_io_type_enum_name {
            $(
                $(#[doc = $doc])?
                $cortical_type_key_name
            ),*
        }

        impl std::fmt::Display for $cortical_io_type_enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ch = match self {
                    $(
                        Self::$cortical_type_key_name => $display_name
                    ),*
                };
                write!(f, "{}", ch)
            }
        }

        impl $cortical_io_type_enum_name {

            pub fn list_all_sensor_types() -> Vec<$cortical_io_type_enum_name> {
                let mut output: Vec<$cortical_io_type_enum_name> = Vec::new();
                    $(
                        output.push($cortical_io_type_enum_name::$cortical_type_key_name);
                    )*
                output
            }
            
            // Does no cortical ID checking
            pub(crate) fn get_type_from_bytes(id: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<CorticalType, FeagiDataProcessingError> {
                let mut id_0: [u8; CorticalID::CORTICAL_ID_LENGTH] = id.clone();
                //id_0.clone_from_slice(id);
                const ZERO_AS_ASCII_BYTE: u8 = 48;
                id_0[4] = ZERO_AS_ASCII_BYTE;
                id_0[5] = ZERO_AS_ASCII_BYTE;

                match &id_0 {
                    $(
                        $base_ascii => Ok((Self::$cortical_type_key_name).into())
                    ),*,
                    _ => return Err(FeagiDataProcessingError::InternalError("Failed to map cortical ID to type!".into()))
                }
                

            }

            pub fn to_cortical_id(&self, index: CorticalGroupingIndex) -> CorticalID {
                let (high, low) = u8_to_hex_char_u8(index.0);
                let mut output: [u8; CorticalID::CORTICAL_ID_LENGTH] =  match self {
                    $(
                        Self::$cortical_type_key_name => *$base_ascii
                    ),*,
                };
                output[4] = high;
                output[5] = low;
                CorticalID {bytes: output} // skip safety checks, we know this is fine
            }

            pub fn get_channel_dimension_range(&self) -> SingleChannelDimensionRange {
                match self {
                    $(
                        Self::$cortical_type_key_name => $channel_dimension_range.unwrap()
                    ),*
                }
            }
            
            pub fn get_coder_type(&self) -> Result<NeuronCoderVariantType, FeagiDataProcessingError> {
                match self {
                    $(
                        Self::$cortical_type_key_name => Ok($default_coder_type)
                    ),*
                }
            }
            

            
        }
    }
}

/// Main classification enum for all types of cortical areas in FEAGI.
///
/// `CorticalType` provides a unified type system for classifying and managing
/// different kinds of cortical areas within the FEAGI neural system. Each variant
/// represents a different category of neural processing with specific properties,
/// constraints, and capabilities.
///
/// # Type Categories
///
/// ## Core Areas
/// Universal cortical areas found in every FEAGI Genomes:
/// - **Death**: Triggers brain death
/// - **Power**: Is always powered
///
/// ## Sensory Areas (Input)
/// Process incoming data from various inputs
///
/// ## Motor Areas (Output)
/// Control outgoing signals to actuators
///
/// ## Custom Areas
/// User-defined processing areas where main neural computation occurs
///
/// ## Memory Areas
/// Persistent storage and retrieval
///
/// # Properties and Capabilities
///
/// # Sensor and Motor types define the following:
/// - **Channel dimensions**: Spatial constraints for neural organization
/// - **Coder types**: Neural encoding/decoding methods (for I/O areas)
///
/// # Usage Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::genomic_structures::*;
///
/// // Check cortical type properties
/// let cortical_type = CorticalType::Sensory(SensorCorticalType::ImageCameraCenter);
/// 
/// // Get dimensional constraints
/// let channel_range = cortical_type.try_get_channel_size_boundaries().unwrap();
/// 
/// // Type classification checks
/// assert!(cortical_type.is_type_sensor());
/// assert!(!cortical_type.is_type_core());
/// ```
///
/// # Design Principles
///
/// - **Type Safety**: Prevents mixing incompatible cortical area types
/// - **Extensibility**: New sensor and motor types can be added via macros
/// - **Validation**: Ensures cortical areas conform to type-specific rules
/// - **Properties**: Each type provides relevant configuration constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorticalType {
    Custom,
    Memory,
    Core(CoreCorticalType),
    Sensory(SensorCorticalType),
    Motor(MotorCorticalType),
}

impl fmt::Display for CorticalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            Self::Custom => write!(f, "CorticalType<Custom>"),
            Self::Memory => write!(f, "CorticalType<Memory>"),
            Self::Core(c) => write!(f, "CorticalType<Core>({})", c),
            Self::Sensory(s) => write!(f, "CorticalType<Sensory>({})", s),
            Self::Motor(m) => write!(f, "CorticalType<Motor>({})", m),
        }
    }
}

impl CorticalType {
    
    /// Determines the cortical type from a cortical ID's bytes representation.
    ///
    /// Analyzes the first bytes of a cortical ID to determine which type category
    /// it belongs to, then delegates to the appropriate subtype parser for
    /// detailed classification.
    ///
    /// # Arguments
    /// * `bytes` - 6-bytes array representing a cortical ID
    ///
    /// # Returns
    /// * `Ok(CorticalType)` - Successfully identified cortical type
    /// * `Err(FeagiDataProcessingError)` - Invalid or unrecognized format
    ///
    /// # Format Detection
    /// - `c` → Custom cortical area
    /// - `m` → Memory cortical area  
    /// - `_` → Core cortical area (delegates to CoreCorticalType)
    /// - `i` → Sensory cortical area (delegates to SensorCorticalType)
    /// - `o` → Motor cortical area (delegates to MotorCorticalType)
    pub fn try_get_type_from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<CorticalType, FeagiDataProcessingError> {
        let start: u8 = bytes[0];
        match start { 
            b'c' => Ok(CorticalType::Custom),
            b'm' => Ok(CorticalType::Memory),
            b'_' => CoreCorticalType::get_type_from_bytes(bytes),
            b'i' => SensorCorticalType::get_type_from_bytes(bytes),
            b'o' => MotorCorticalType::get_type_from_bytes(bytes),
            _ => Err(handle_byte_id_mapping_fail(bytes))
        }
        
    }
    
    /// Attempts to convert the cortical type to a CorticalID, using the io_cortical_index in the
    /// case of input / output cortical areas as well
    /// 
    /// Uses internal enum table lookups to mainly accomplish this. Generally using this
    /// method of generating Cortical IDs is preferred over memorizing the strings of each 
    /// Cortical Type.
    /// 
    /// This function does not work with finding memory or custom types, as those cortical IDs are
    /// largely arbitrary.
    ///
    /// # Returns
    /// * `Ok(CorticalID)` - Successfully identified cortical id
    /// * `Err(FeagiDataProcessingError)` - In the case of trying to convert custom or memory types
    pub fn to_cortical_id(&self, io_cortical_index: CorticalGroupingIndex) -> Result<CorticalID, FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas can have arbitrary Cortical IDs and thus cannot be convert to from type!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas can have arbitrary Cortical IDs and thus cannot be convert to from type!".into()).into()),
            Self::Core(c) => {
                Ok(CorticalID::new_core_cortical_area_id(*c)?)
            }
            Self::Sensory(s) => {
                Ok(CorticalID::new_sensor_cortical_area_id(*s, io_cortical_index)?)
            }
            Self::Motor(m) => {
                Ok(CorticalID::new_motor_cortical_area_id(*m, io_cortical_index)?)
            }
        }
        
    }

    /// Returns the dimensional constraints for channels of this cortical type.
    ///
    /// Different cortical types have different dimensional requirements for their
    /// internal channels. This method returns the valid range for X, Y, and Z
    /// dimensions that channels of this type must conform to.
    ///
    /// # Returns
    /// * `Ok(SingleChannelDimensionRange)` - Valid dimensional constraints for this type
    /// * `Err(FeagiDataProcessingError)` - For Custom and Memory types (no channels)
    ///
    /// # Channel Dimensions by Type
    /// - **Core**: Fixed small dimensions (typically 1x1x1)
    /// - **Sensory**: Variable, often large (e.g., vision: up to 4096x4096x4)
    /// - **Motor**: Usually small and specific (e.g., 1x1x8 for multi-axis)
    /// - **Custom/Memory**: No channels (return error)
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, SensorCorticalType, SingleChannelDimensions};
    /// let vision_type = CorticalType::Sensory(SensorCorticalType::Proximity);
    /// let constraints = vision_type.try_get_channel_size_boundaries().unwrap();
    ///
    /// // Validate proposed dimensions against constraints
    /// let proposed = SingleChannelDimensions::new(1, 1, 3).unwrap();
    /// constraints.verify_within_range(&proposed).unwrap();
    /// ```
    pub fn try_get_channel_size_boundaries(&self) -> Result<SingleChannelDimensionRange, FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas do not have channels!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas do not have channels!".into()).into()),
            Self::Core(_) => Err(IODataError::InvalidParameters("Core Cortical Areas do not have channels!".into()).into()),
            Self::Sensory(s) => Ok(s.get_channel_dimension_range()),
            Self::Motor(m) => Ok(m.get_channel_dimension_range()),
        }
    }
    
    /// Returns the neural coder type used for data encoding/decoding.
    ///
    /// I/O cortical areas (Sensory and Motor) use specific neural coding schemes
    /// to convert between external data formats and internal neural representations.
    /// This method returns the appropriate coder for the cortical type.
    ///
    /// # Returns
    /// * `Ok(NeuronCoderVariantType)` - Neural coder used by this I/O type
    /// * `Err(FeagiDataProcessingError)` - For non-I/O types (Custom, Memory, Core)
    ///
    /// # Coder Types by Application
    /// - **Vision sensors**: Often use normalized float coders for pixel values
    /// - **Motor outputs**: May use signed normalized coders for bidirectional control
    /// - **Audio sensors**: Frequency domain or time domain coders
    /// - **Other I/O**: Type-specific encoding optimized for the data characteristics
    pub(crate) fn try_get_coder_type(&self) -> Result<NeuronCoderVariantType, FeagiDataProcessingError> {
        match self {
            Self::Custom => Err(IODataError::InvalidParameters("Custom Cortical Areas do not have coders!".into()).into()),
            Self::Memory => Err(IODataError::InvalidParameters("Memory Cortical Areas do not have coders!".into()).into()),
            Self::Core(_) => Err(IODataError::InvalidParameters("Core Cortical Areas do not have coders!".into()).into()),
            Self::Sensory(s) => s.get_coder_type(),
            Self::Motor(m) => m.get_coder_type(),
        }
    }
    
    /// Checks if this cortical type is a core system area.
    ///
    /// # Returns
    /// `true` if this is a `CorticalType::Core` variant, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, CoreCorticalType};
    /// 
    /// let core_type = CorticalType::Core(CoreCorticalType::Power);
    /// assert!(core_type.is_type_core());
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(!custom_type.is_type_core());
    /// ```
    pub fn is_type_core(&self) -> bool {
        match self {
            Self::Core(_) => true,
            _ => false
        }
    }

    /// Checks if this cortical type is a sensory input area.
    ///
    /// # Returns
    /// `true` if this is a `CorticalType::Sensory` variant, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, MotorCorticalType, SensorCorticalType};
    ///
    /// let sensor_type = CorticalType::Sensory(SensorCorticalType::ImageCameraCenter);
    /// assert!(sensor_type.is_type_sensor());
    ///
    /// let motor_type = CorticalType::Motor(MotorCorticalType::RotaryMotor);
    /// assert!(!motor_type.is_type_sensor());
    /// ```
    pub fn is_type_sensor(&self) -> bool {
        match self {
            Self::Sensory(_) => true,
            _ => false
        }
    }

    /// Checks if this cortical type is a motor output area.
    ///
    /// # Returns
    /// `true` if this is a `CorticalType::Motor` variant, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, MotorCorticalType};
    /// 
    /// let motor_type = CorticalType::Motor(MotorCorticalType::RotaryMotor);
    /// assert!(motor_type.is_type_motor());
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(!custom_type.is_type_motor());
    /// ```
    pub fn is_type_motor(&self) -> bool {
        match self {
            Self::Motor(_) => true,
            _ => false
        }
    }

    /// Checks if this cortical type is a custom processing area.
    ///
    /// # Returns
    /// `true` if this is a `CorticalType::Custom` variant, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalType;
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(custom_type.is_type_custom());
    /// 
    /// let memory_type = CorticalType::Memory;
    /// assert!(!memory_type.is_type_custom());
    /// ```
    pub fn is_type_custom(&self) -> bool {
        match self {
            Self::Custom => true,
            _ => false
        }
    }

    /// Checks if this cortical type is a memory storage area.
    ///
    /// # Returns
    /// `true` if this is a `CorticalType::Memory` variant, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalType;
    /// 
    /// let memory_type = CorticalType::Memory;
    /// assert!(memory_type.is_type_memory());
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(!custom_type.is_type_memory());
    /// ```
    pub fn is_type_memory(&self) -> bool {
        match self {
            Self::Memory => true,
            _ => false
        }
    }
    
    /// Verifies that this cortical type is a core system area, returning an error if not.
    ///
    /// # Returns
    /// * `Ok(())` - If this is a `CorticalType::Core` variant
    /// * `Err(FeagiDataProcessingError)` - If this is any other type
    ///
    /// # Errors
    /// Returns `IODataError::InvalidParameters` if the cortical type is not core.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, CoreCorticalType};
    /// 
    /// let core_type = CorticalType::Core(CoreCorticalType::Death);
    /// assert!(core_type.verify_is_core().is_ok());
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(custom_type.verify_is_core().is_err());
    /// ```
    pub fn verify_is_core(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_core() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Core!".into()).into())
        }
        Ok(())
    }

    /// Verifies that this cortical type is a sensory input area, returning an error if not.
    ///
    /// # Returns
    /// * `Ok(())` - If this is a `CorticalType::Sensory` variant
    /// * `Err(FeagiDataProcessingError)` - If this is any other type
    ///
    /// # Errors
    /// Returns `IODataError::InvalidParameters` if the cortical type is not sensory.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, MotorCorticalType, SensorCorticalType};
    ///
    /// let sensor_type = CorticalType::Sensory(SensorCorticalType::Proximity);
    /// assert!(sensor_type.verify_is_sensor().is_ok());
    ///
    /// let motor_type = CorticalType::Motor(MotorCorticalType::RotaryMotor);
    /// assert!(motor_type.verify_is_sensor().is_err());
    /// ```
    pub fn verify_is_sensor(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_sensor() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Sensor!".into()).into())
        }
        Ok(())
    }
    
    /// Verifies that this cortical type is a motor output area, returning an error if not.
    ///
    /// # Returns
    /// * `Ok(())` - If this is a `CorticalType::Motor` variant
    /// * `Err(FeagiDataProcessingError)` - If this is any other type
    ///
    /// # Errors
    /// Returns `IODataError::InvalidParameters` if the cortical type is not motor.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::{CorticalType, MotorCorticalType};
    /// 
    /// let motor_type = CorticalType::Motor(MotorCorticalType::RotaryMotor);
    /// assert!(motor_type.verify_is_motor().is_ok());
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(custom_type.verify_is_motor().is_err());
    /// ```
    pub fn verify_is_motor(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_motor() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Motor!".into()).into())
        }
        Ok(())
    }

    /// Verifies that this cortical type is a custom processing area, returning an error if not.
    ///
    /// # Returns
    /// * `Ok(())` - If this is a `CorticalType::Custom` variant
    /// * `Err(FeagiDataProcessingError)` - If this is any other type
    ///
    /// # Errors
    /// Returns `IODataError::InvalidParameters` if the cortical type is not custom.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalType;
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(custom_type.verify_is_custom().is_ok());
    /// 
    /// let memory_type = CorticalType::Memory;
    /// assert!(memory_type.verify_is_custom().is_err());
    /// ```
    pub fn verify_is_custom(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_custom() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Custom!".into()).into())
        }
        Ok(())
    }

    /// Verifies that this cortical type is a memory storage area, returning an error if not.
    ///
    /// # Returns
    /// * `Ok(())` - If this is a `CorticalType::Memory` variant
    /// * `Err(FeagiDataProcessingError)` - If this is any other type
    ///
    /// # Errors
    /// Returns `IODataError::InvalidParameters` if the cortical type is not memory.
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CorticalType;
    /// 
    /// let memory_type = CorticalType::Memory;
    /// assert!(memory_type.verify_is_memory().is_ok());
    /// 
    /// let custom_type = CorticalType::Custom;
    /// assert!(custom_type.verify_is_memory().is_err());
    /// ```
    pub fn verify_is_memory(&self) -> Result<(), FeagiDataProcessingError> {
        if !self.is_type_memory() {
            return Err(IODataError::InvalidParameters("Expected cortical type to be type Memory!".into()).into())
        }
        Ok(())
    }
}

//region Core

/// Core system cortical areas for essential FEAGI operations.
///
/// Core cortical areas are in all genomes and can always be expected to exist
///
/// # Core Area Types
///
/// ## Death
/// When activated, causes Brain Death
///
/// ## Power
/// Is always on
///
/// # Characteristics
/// - **Fixed IDs**: Core areas have predetermined cortical identifiers
/// - **Universal**: Required for proper FEAGI operation  
/// - **No Indexing**: Only one instance of each core type per system
/// - **Minimal Dimensions**: 1x1x1 spatial structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoreCorticalType {
    Death,
    Power
}

impl fmt::Display for CoreCorticalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            CoreCorticalType::Death => "Death",
            CoreCorticalType::Power => "Power"
        };
        write!(f, "CoreCorticalType({})", ch)
    }
}

impl From<CoreCorticalType> for CorticalType {
    fn from(core: CoreCorticalType) -> Self {
        CorticalType::Core(core)
    }
}

impl From<&CoreCorticalType> for CorticalType {
    fn from(core: &CoreCorticalType) -> Self {
        CorticalType::Core(*core)
    }
}

impl CoreCorticalType {

    /// Converts this core type to its corresponding cortical identifier.
    ///
    /// Core cortical areas have fixed, predefined identifiers that start with
    /// an underscore prefix. These identifiers are system-reserved and cannot
    /// be used for other cortical area types.
    ///
    /// # Returns
    /// The fixed `CorticalID` for this core type
    ///
    /// # Core Identifiers
    /// - `Death` → `"_death"`
    /// - `Power` → `"_power"`
    ///
    /// # Example
    /// ```rust
    /// use feagi_core_data_structures_and_processing::genomic_structures::CoreCorticalType;
    /// let death_id = CoreCorticalType::Death.to_cortical_id();
    /// assert_eq!(death_id.as_ascii_string(), "_death");
    /// ```
    pub fn to_cortical_id(&self) -> CorticalID {
        match self {
            Self::Death => CorticalID{bytes: *b"_death"},
            Self::Power => CorticalID{bytes: *b"_power"},
        }
    }

    pub(crate) fn get_type_from_bytes(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> Result<CorticalType, FeagiDataProcessingError> {
        match bytes {
            b"_death" => Ok(CoreCorticalType::Death.into()),
            b"_power" => Ok(CoreCorticalType::Power.into()),
            _ => Err(handle_byte_id_mapping_fail(bytes)),
        }
    }

}

//endregion

//region Sensor Cortical Area types

// Use sensor_definition directly with callback pattern
sensor_definition!(define_io_cortical_types);

impl From<SensorCorticalType> for CorticalType {
    fn from(input: SensorCorticalType) -> Self {
        CorticalType::Sensory(input)
    }
}

//endregion

//region Motor Cortical Area types

define_io_cortical_types!{
    MotorCorticalType {
        RotaryMotor => {
            friendly_name: "Rotary Motor",
            base_ascii: b"omot00",
            channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..u32::MAX),
            default_coder_type: NeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided,
        },
    }    
}

impl From<MotorCorticalType> for CorticalType {
    fn from(input: MotorCorticalType) -> Self {
        CorticalType::Motor(input)
    }
}

//endregion

//region helpers
fn hex_chars_to_u8(high: char, low: char) -> Result<u8, GenomeError> {
    fn hex_value(c: char) -> Result<u8, GenomeError> {
        match c {
            '0'..='9' => Ok(c as u8 - b'0'),
            'a'..='f' => Ok(c as u8 - b'a' + 10),
            'A'..='F' => Ok(c as u8 - b'A' + 10),
            _ => Err(GenomeError::InvalidCorticalID(format!("Index of '{}' is not a valid hexadecimal!", c))),
        }
    }
    let hi = hex_value(high)?;
    let lo = hex_value(low)?;

    Ok((hi << 4) | lo)
}

fn u8_to_hex_char_u8(index: u8) -> (u8, u8) {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

    let high = HEX_CHARS[(index >> 4) as usize];
    let low = HEX_CHARS[(index & 0x0F) as usize];

    (high, low)
}

// Used when we know something is wrong, we just want the right error
fn handle_byte_id_mapping_fail(bytes: &[u8; CorticalID::CORTICAL_ID_LENGTH]) -> FeagiDataProcessingError {
    let as_string = String::from_utf8(bytes.to_vec());
    if as_string.is_err() {
        FeagiBytesError::UnableToDeserializeBytes("Unable to parse cortical ID as ASCII!".into()).into()
    }
    else {
        GenomeError::InvalidCorticalID(format!("Invalid cortical ID '{}'!", as_string.unwrap())).into()
    }
}
//endregion
