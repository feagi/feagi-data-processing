//! Error handling for FEAGI data processing operations.
//!
//! This module defines the centralized error type [`DataProcessingError`] used throughout 
//! the FEAGI data processing library. It provides comprehensive error handling for various
//! failure modes that can occur during data processing operations.
//! ```

use std::error::Error;
use std::fmt;

//region Top Level
#[derive(Debug)]
pub enum FeagiDataProcessingError {
    IOData(IODataError),
    FeagiBytes(FeagiBytesError),
    IODevice(IODeviceError),
    NeuronData(NeuronError),
    Genome(GenomeError),
    CommandAndControl(CommandAndControlError),
    InternalError(String),
    NotImplemented,
}

impl From<IODataError> for FeagiDataProcessingError {
    fn from(err: IODataError) -> Self {
        FeagiDataProcessingError::IOData(err)
    }
}

impl From<FeagiBytesError> for FeagiDataProcessingError {
    fn from(err: FeagiBytesError) -> Self {
        FeagiDataProcessingError::FeagiBytes(err)
    }
}

impl From<IODeviceError> for FeagiDataProcessingError {
    fn from(err: IODeviceError) -> Self {
        FeagiDataProcessingError::IODevice(err) 
    }
}

impl From<NeuronError> for FeagiDataProcessingError {
    fn from(err: NeuronError) -> Self {
        FeagiDataProcessingError::NeuronData(err)
    }
}

impl From<GenomeError> for FeagiDataProcessingError {
    fn from(err: GenomeError) -> Self {
        FeagiDataProcessingError::Genome(err)
    }
}

impl From<CommandAndControlError> for FeagiDataProcessingError {
    fn from(err: CommandAndControlError) -> Self {
        FeagiDataProcessingError::CommandAndControl(err)
    }
}

impl fmt::Display for FeagiDataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeagiDataProcessingError::IOData(e) => write!(f, "IO Data Error: {}", e),
            FeagiDataProcessingError::FeagiBytes(e) => write!(f, "Byte Processing Error: {}", e),
            FeagiDataProcessingError::IODevice(e) => write!(f, "IO Device Error: {}", e),
            FeagiDataProcessingError::NeuronData(e) => write!(f, "Error Processing Neuron Data: {}", e),
            FeagiDataProcessingError::Genome(e) => write!(f, "Error Processing Genome Data: {}", e),
            FeagiDataProcessingError::CommandAndControl(e) => write!(f, "Command/Control Error: {}", e),
            FeagiDataProcessingError::InternalError(e) => write!(f, "Internal Error, please raise an issue on Github!: {}", e),
            FeagiDataProcessingError::NotImplemented => write!(f, "This function is not yet implemented! Please raise an issue on Github!"),
        }
    }
}

impl Error for FeagiDataProcessingError {}
//endregion

//region Specific Error Subtypes

#[derive(Debug)]
pub enum IODataError{
    InvalidParameters(String),
    InvalidInplaceOperation(String),
}

impl fmt::Display for IODataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            IODataError::InvalidParameters(e) => write!(f, "Invalid parameters: {}", e),
            IODataError::InvalidInplaceOperation(e) => write!(f, "Invalid inplace operation: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum FeagiBytesError {
    UnableToValidateBytes(String),
    UnableToSerializeBytes(String),
    UnableToDeserializeBytes(String),
}

impl fmt::Display for FeagiBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            FeagiBytesError::UnableToValidateBytes(e) => write!(f, "Unable to validate bytes: {}", e),
            FeagiBytesError::UnableToSerializeBytes(e) => write!(f, "Unable to serialize bytes: {}", e),
            FeagiBytesError::UnableToDeserializeBytes(e) => write!(f, "Unable to deserialize bytes: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum IODeviceError {
    InvalidSensorInputValues(String),
    InvalidMotorCallback(String),
    ReceivedInvalidFEAGIMotorData(String),
}

impl fmt::Display for IODeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IODeviceError::InvalidSensorInputValues(e) => write!(f, "Received invalid sensor input values: {}", e),
            IODeviceError::InvalidMotorCallback(e) => write!(f, "Invalid motor callback function: {}", e),
            IODeviceError::ReceivedInvalidFEAGIMotorData(e) => write!(f, "FEAGI has sent invalid Motor Data: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum NeuronError {
    UnableToParseFromNeuronData(String),
    UnableToConvertNeuronFormat(String),
    UnableToGenerateNeuronData(String),
}

impl fmt::Display for NeuronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeuronError::UnableToParseFromNeuronData(e) => write!(f, "Unable to parse neuron data: {}", e),
            NeuronError::UnableToConvertNeuronFormat(e) => write!(f, "Unable to convert neuron data: {}", e),
            NeuronError::UnableToGenerateNeuronData(e) => write!(f, "Unable to generate neuron data: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum GenomeError {
    InvalidCorticalID(String),
    InvalidCorticalDimensions(String),
    InvalidChannelDimensions(String),
}

impl fmt::Display for GenomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenomeError::InvalidCorticalID(e) => write!(f, "Unable to Process Cortical ID: {}", e),
            GenomeError::InvalidCorticalDimensions(e) => write!(f, "Invalid Cortical Dimensions: {}", e),
            GenomeError::InvalidChannelDimensions(e) => write!(f, "Invalid Channel Dimensions: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum CommandAndControlError {
    InvalidCommandParameter(String),
    InvalidResponse(String),
    NoResponse(String),
    AuthenticationFailure(String),
}

impl fmt::Display for CommandAndControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandAndControlError::InvalidCommandParameter(e) => write!(f, "Invalid command parameter: {}", e),
            CommandAndControlError::InvalidResponse(e) => write!(f, "Received Invalid response: {}", e),
            CommandAndControlError::NoResponse(e) => write!(f, "No response: {}", e),
            CommandAndControlError::AuthenticationFailure(e) => write!(f, "Authentication failure: {}", e),
        }
    }
}

impl Error for IODataError {}
impl Error for FeagiBytesError {}
impl Error for IODeviceError {}
impl Error for NeuronError {}
impl Error for GenomeError {}
impl Error for CommandAndControlError {}



//endregion