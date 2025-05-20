use crate::Error::DataProcessingError;

/// Length of Cortical Area ID As ASCII characters / bytes
const CORTICAL_ID_LENGTH: usize = 6;

/// Represents an ID for a cortical area in the brain
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CorticalID {
    /// The raw byte representation of the cortical identifier
    id: [u8; CORTICAL_ID_LENGTH]
}

impl CorticalID {
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
        if id.len() != CORTICAL_ID_LENGTH {
            return Err(DataProcessingError::InvalidInputBounds("Cortical Area ID Incorrect Length!".into()));
        }
        let bytes = id.as_bytes();
        
        if !bytes.iter().all(|&b| b.is_ascii()) {
            return Err(DataProcessingError::InvalidInputBounds("Cortical ID must contain only ASCII characters!".into()));
        }

        let mut inner = [0u8; CORTICAL_ID_LENGTH];
        inner.copy_from_slice(bytes);
        Ok(CorticalID { id: inner })
    }
    
    /// Creates a new CorticalID from a subset of bytes starting at a given offset
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte slice containing the cortical identifier
    /// * `offset` - The starting position within the byte slice
    ///
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
    pub fn from_bytes_at(bytes: &[u8], offset: usize) -> Result<CorticalID, DataProcessingError> { // TODO do not use offsets
        if offset + CORTICAL_ID_LENGTH > bytes.len() {
            return Err(DataProcessingError::InvalidInputBounds("Not enough bytes starting from the given offset!".into()));
        }

        let slice = &bytes[offset..offset + CORTICAL_ID_LENGTH];

        if !slice.iter().all(|&b| b.is_ascii()) {
            return Err(DataProcessingError::InvalidInputBounds("Cortical ID must contain only ASCII characters!".into()));
        }

        let mut inner = [0u8; CORTICAL_ID_LENGTH];
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
    
    pub fn write_bytes_at(&self, bytes_to_write_at: &mut [u8]) -> Result<(), DataProcessingError> {
        if bytes_to_write_at.len() != CORTICAL_ID_LENGTH {
            return Err(DataProcessingError::InvalidInputBounds("Cortical Area ID need a length of exactly 6 bytes to fit!".into()));
        };
        bytes_to_write_at.copy_from_slice(&self.id);
        Ok(())
    }
    
}