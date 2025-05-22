use std::fmt;

#[derive(Debug)]
pub enum DataProcessingError {
    InvalidInputBounds(String),
    IncompatibleInplace(String),
    IncompatibleInputArray(String),
    InternalError(String),
    MissingContext(String),
    NotImplemented,
}

impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            DataProcessingError::InvalidInputBounds(e) => write!(f, "Invalid Bounds! {}", e),
            DataProcessingError::IncompatibleInplace(e) => write!(f, "Unable to perform inplace operation! {}", e),
            DataProcessingError::IncompatibleInputArray(e) => write!(f, "Incompatible input array for images! {}", e),
            DataProcessingError::InternalError(e) => write!(f, "Internal error! Please report this! {}", e),
            DataProcessingError::MissingContext(e) => write!(f, "Missing context! {}", e),
            DataProcessingError::NotImplemented => write!(f, "This function is not yet implemented! Please raise an issue on Github!"),
        }
    }
}