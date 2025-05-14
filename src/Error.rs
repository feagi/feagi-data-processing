use std::fmt;

#[derive(Debug)]
pub enum DataProcessingError {
    InvalidInputBounds(String),
}

impl fmt::Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            DataProcessingError::InvalidInputBounds(e) => write!(f, "Invalid Bounds! {}", e),
        }
    }
}