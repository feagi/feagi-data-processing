//! Cortical area identification and data structures for FEAGI.
//! 
//! This module provides the `CorticalID` type for identifying cortical areas in the FEAGI
//! brain simulation. Cortical IDs are fixed-length ASCII identifiers that map to specific
//! brain regions and are used throughout the system for organizing and routing neuron data.

use crate::error::DataProcessingError;

/*
pub enum CorticalAreaType {
    Motor(MotorType),
    Sensor(SensorType)
}

pub enum MotorType {
    Servo,
}

pub enum SensorType {
    Battery,
    Camera(CameraType)
}

pub enum CameraType {
    Center,
    LowerLeft,
    MiddleLeft,
    UpperLeft,
    UpperMiddle,
    UpperRight,
    MiddleRight,
    LowerRight,
    LowerMiddle,
}

 */


/// Represents an ID for a cortical area in the brain
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CorticalID {
    /// The raw byte representation of the cortical identifier
    id: [u8; CorticalID::CORTICAL_ID_LENGTH]
}

impl CorticalID {
    /// Length of Cortical Area ID As ASCII characters / bytes
    pub const CORTICAL_ID_LENGTH: usize = 6;
    
    /// Creates a new CorticalID from a string representation
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the cortical identifier
    ///
    /// # Returns
    ///
    /// * `Result<CorticalID, &'static str>` - A Result containing either the constructed CorticalID
    ///   or an error message if the input is invalid
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The input string length doesn't match CORTICAL_ID_LENGTH
    /// * The input string contains non-ASCII characters
    pub fn from_str(id: &str) -> Result<CorticalID, DataProcessingError> {
        if id.len() != CorticalID::CORTICAL_ID_LENGTH {
            return Err(DataProcessingError::InvalidInputBounds("Cortical Area ID Incorrect Length!".into()));
        }
        let bytes = id.as_bytes();
        
        if !bytes.iter().all(|&b| b.is_ascii()) {
            return Err(DataProcessingError::InvalidInputBounds("Cortical ID must contain only ASCII characters!".into()));
        }

        let mut inner = [0u8; CorticalID::CORTICAL_ID_LENGTH];
        inner.copy_from_slice(bytes);
        Ok(CorticalID { id: inner })
    }
    
    /// Creates a new CorticalID from a subset of bytes starting at a given offset
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte slice containing the cortical identifier

    /// # Returns
    ///
    /// * `Result<CorticalID, &'static str>` - A Result containing either the constructed CorticalID
    ///   or an error message if the input is invalid
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * There aren't enough bytes available from the offset
    /// * The bytes contain non-ASCII characters
    pub fn from_bytes_at(bytes: &[u8]) -> Result<CorticalID, DataProcessingError> {
        if CorticalID::CORTICAL_ID_LENGTH != bytes.len() {
            return Err(DataProcessingError::InvalidInputBounds(format!("Expected exactly {} bytes for getting the cortical ID! Received a slice of {} bytes!", CorticalID::CORTICAL_ID_LENGTH, bytes.len()).into()));
        }
        

        if !bytes.iter().all(|&b| b.is_ascii()) {
            return Err(DataProcessingError::InvalidInputBounds("Cortical ID must contain only ASCII characters!".into()));
        }

        let mut inner = [0u8; CorticalID::CORTICAL_ID_LENGTH];
        inner.copy_from_slice(bytes);
        Ok(CorticalID { id: inner })
    }
    

    /// Converts the CorticalID to a string slice
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice representing the cortical identifier
    ///
    /// # Panics
    ///
    /// This function should never panic as the CorticalID is guaranteed to contain
    /// only valid ASCII characters by its constructors.
    pub fn as_str(&self) -> &str {
        // Safe because we validate in the constructor that it's ASCII
        std::str::from_utf8(&self.id).unwrap()
    }
    
    /// Writes the cortical ID bytes to a target byte slice.
    /// 
    /// This method copies the cortical ID's internal byte representation to the
    /// provided byte slice. The target slice must be exactly the correct length
    /// to hold the cortical ID.
    /// 
    /// # Arguments
    /// 
    /// * `bytes_to_write_at` - The target byte slice to write the cortical ID to
    /// 
    /// # Returns
    /// 
    /// A Result containing either:
    /// - Ok(()) if the write was successful
    /// - Err(DataProcessingError) if the target slice has incorrect length
    /// 
    /// # Examples
    /// 
    /// ```
    /// use feagi_core_data_structures_and_processing::cortical_data::CorticalID;
    /// 
    /// let cortical_id = CorticalID::from_str("iv00CC").unwrap();
    /// let mut buffer = [0u8; 6];
    /// cortical_id.write_bytes_at(&mut buffer).unwrap();
    /// assert_eq!(&buffer, b"iv00CC");
    /// ```
    pub fn write_bytes_at(&self, bytes_to_write_at: &mut [u8]) -> Result<(), DataProcessingError> {
        if bytes_to_write_at.len() != CorticalID::CORTICAL_ID_LENGTH {
            return Err(DataProcessingError::InvalidInputBounds(format!("Cortical Area ID need a length of exactly {} bytes to fit!", CorticalID::CORTICAL_ID_LENGTH).into()));
        };
        bytes_to_write_at.copy_from_slice(&self.id);
        Ok(())
    }
    
}