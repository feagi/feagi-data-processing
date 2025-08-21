//! Error handling for FEAGI data processing operations.
//!
//! This module defines the centralized error handling system for the FEAGI data processing
//! library. It provides comprehensive error types for various failure modes that can occur
//! during data processing operations.
//!
//! # Error Hierarchy
//!
//! The error system is built around a main error type [`FeagiDataProcessingError`] that 
//! encompasses all possible errors in the library. This top-level error contains specific
//! error subtypes for different domains:
//!
//! - [`IODataError`] - Input/output data validation and processing errors
//! - [`FeagiBytesError`] - Byte serialization/deserialization errors  
//! - [`IODeviceError`] - Hardware device and sensor/motor errors
//! - [`NeuronError`] - Neuron data processing and format conversion errors
//! - [`GenomeError`] - Genome structure and cortical area errors
//! - [`CommandAndControlError`] - Command/control protocol and authentication errors
//!
//! # Usage
//!
//! The error types implement standard Rust error handling patterns:
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
//!
//! fn example_function() -> Result<(), FeagiDataProcessingError> {
//!     // IO errors are automatically converted to the main error type
//!     Err(IODataError::InvalidParameters("Bad input".to_string()))?
//! }
//!
//! fn handle_errors() {
//!     match example_function() {
//!         Ok(()) => println!("Success!"),
//!         Err(FeagiDataProcessingError::IOData(io_err)) => {
//!             eprintln!("IO error: {}", io_err);
//!         }
//!         Err(other) => eprintln!("Other error: {}", other),
//!     }
//! }
//! ```
//!
//! # Error Categories
//!
//! ## Data Processing Errors ([`IODataError`])
//! Used for invalid input parameters, failed data transformations, and invalid in-place operations.
//! Common in sensor data processing and cortical area management.
//!
//! ## Byte Processing Errors ([`FeagiBytesError`])  
//! Used for serialization/deserialization failures, bytes validation errors, and incompatible
//! bytes structure usage. Critical for network communication and data persistence.
//!
//! ## Device Errors ([`IODeviceError`])
//! Used for sensor input validation failures, motor callback errors, and invalid motor data
//! from FEAGI. Essential for embodiment and hardware integration.
//!
//! ## Neuron Processing Errors ([`NeuronError`])
//! Used for neuron data parsing failures, format conversion errors, and neuron data generation
//! issues. Important for brain state processing.
//!
//! ## Genome Processing Errors ([`GenomeError`])
//! Used for invalid cortical IDs, dimension validation failures, and channel configuration
//! errors. Critical for brain architecture validation.
//!
//! ## Command/Control Errors ([`CommandAndControlError`])
//! Used for invalid command parameters, authentication failures, and communication protocol
//! errors. Essential for system coordination and security.
//!
//! # Design Principles
//!
//! - **Centralized**: All errors go through [`FeagiDataProcessingError`] for consistent handling
//! - **Contextual**: Each error variant includes descriptive messages with relevant context
//! - **Automatic conversion**: `From` implementations allow `?` operator usage across error types
//! - **Standard compliance**: Implements `std::error::Error` and `Display` for ecosystem compatibility

use std::error::Error;
use std::fmt;

//region Top Level

/// Main error type for the FEAGI data processing library.
///
/// This enum encompasses all possible errors that can occur during FEAGI data processing
/// operations. It provides a centralized error handling mechanism with automatic conversion
/// from specific error subtypes.
///
/// # Variants
///
/// Each variant wraps a specific error type for different domains of functionality:
///
/// - [`IOData`](FeagiDataProcessingError::IOData) - Data validation and processing errors
/// - [`FeagiBytes`](FeagiDataProcessingError::FeagiBytes) - Byte serialization/deserialization errors
/// - [`IODevice`](FeagiDataProcessingError::IODevice) - Hardware device and sensor/motor errors  
/// - [`NeuronData`](FeagiDataProcessingError::NeuronData) - Neuron data processing errors
/// - [`Genome`](FeagiDataProcessingError::Genome) - Genome structure and cortical area errors
/// - [`CommandAndControl`](FeagiDataProcessingError::CommandAndControl) - Command/control protocol errors
/// - [`InternalError`](FeagiDataProcessingError::InternalError) - Internal library bugs (please report!)
/// - [`NotImplemented`](FeagiDataProcessingError::NotImplemented) - Unimplemented functionality
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::{FeagiDataProcessingError, IODataError};
///
/// // Automatic conversion from specific error types
/// let error: FeagiDataProcessingError = IODataError::InvalidParameters("bad input".to_string()).into();
///
/// // Pattern matching for error handling
/// match error {
///     FeagiDataProcessingError::IOData(io_err) => eprintln!("Data error: {}", io_err),
///     FeagiDataProcessingError::InternalError(msg) => eprintln!("Internal error: {}", msg),
///     other => eprintln!("Other error: {}", other),
/// }
/// ```
#[derive(Debug)]
pub enum FeagiDataProcessingError {
    /// Input/output data validation and processing errors.
    /// 
    /// Wraps [`IODataError`] for issues with data validation, parameter checking,
    /// and invalid in-place operations.
    IOData(IODataError),
    
    /// Byte processing and serialization errors.
    ///
    /// Wraps [`FeagiBytesError`] for issues with bytes validation, serialization,
    /// deserialization, and incompatible bytes structure usage.
    FeagiBytes(FeagiBytesError),
    
    /// Hardware device and I/O errors.
    ///
    /// Wraps [`IODeviceError`] for issues with sensor inputs, motor outputs,
    /// and device communication.
    IODevice(IODeviceError),
    
    /// Neuron data processing errors.
    ///
    /// Wraps [`NeuronError`] for issues with neuron data parsing, format conversion,
    /// and neuron data generation.
    NeuronData(NeuronError),
    
    /// Genome structure and cortical area errors.
    ///
    /// Wraps [`GenomeError`] for issues with cortical IDs, dimensions,
    /// and channel configurations.
    Genome(GenomeError),
    
    /// Command and control protocol errors.
    ///
    /// Wraps [`CommandAndControlError`] for issues with command parameters,
    /// authentication, and communication protocols.
    CommandAndControl(CommandAndControlError),
    
    /// Internal library errors that indicate bugs.
    ///
    /// These errors should not occur during normal operation and indicate
    /// a bug in the library. Please report these on GitHub with context.
    InternalError(String),
    
    /// Functionality that is not yet implemented.
    ///
    /// Indicates a feature or function that is planned but not yet available.
    /// Please raise an issue on GitHub if you need this functionality.
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

/// Errors related to input/output data validation and processing.
///
/// This error type covers issues that occur during data validation, parameter checking,
/// and data transformation operations. It is commonly used throughout the sensor processing
/// pipeline and cortical area management systems.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::IODataError;
///
/// // Invalid parameter example
/// let error = IODataError::InvalidParameters("Channel index out of bounds".to_string());
///
/// // Invalid operation example  
/// let error = IODataError::InvalidInplaceOperation("Cannot modify immutable data".to_string());
/// ```
#[derive(Debug)]
pub enum IODataError{
    /// Invalid parameters provided to a function or method.
    ///
    /// This variant is used when function arguments are outside expected ranges,
    /// have invalid values, or are incompatible with the current system state.
    /// 
    /// Common cases include:
    /// - Channel indices exceeding cortical area capacity
    /// - Cortical areas already registered
    /// - Zero-channel cortical area registration attempts
    /// - Type mismatches for sensor data
    InvalidParameters(String),
    
    /// Invalid in-place operation on data structures.
    ///
    /// This variant is used when attempting to modify data that should not be
    /// modified in-place, or when the modification would result in an invalid state.
    ///
    /// Common cases include:
    /// - Modifying immutable sensor data
    /// - Invalid transformations on neuron data structures
    /// - Attempting to resize fixed-size data io_containers
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

/// Errors related to bytes processing, serialization, and deserialization.
///
/// This error type covers issues that occur during bytes-level operations, including
/// validation of bytes structures, serialization to binary formats, and deserialization
/// from network or storage sources. Critical for network communication and data persistence.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::FeagiBytesError;
///
/// // Validation error
/// let error = FeagiBytesError::UnableToValidateBytes("Invalid magic bytes".to_string());
///
/// // Serialization error
/// let error = FeagiBytesError::UnableToSerializeBytes("Data too large for format".to_string());
/// ```
#[derive(Debug)]
pub enum FeagiBytesError {
    /// Failed to validate bytes structure or format.
    ///
    /// This variant is used when bytes data doesn't conform to expected formats,
    /// has invalid headers, or fails validation.
    ///
    /// Common cases include:
    /// - Invalid magic bytes or headers
    /// - Checksum validation failures  
    /// - Corrupted bytes sequences
    /// - Unsupported format versions
    UnableToValidateBytes(String),
    
    /// Failed to serialize data to bytes format.
    ///
    /// This variant is used when data structures cannot be converted to their
    /// binary representation due to format limitations or data constraints.
    ///
    /// Common cases include:
    /// - Data exceeds format size limits
    /// - Unsupported data types for format
    /// - Memory allocation failures during serialization
    UnableToSerializeBytes(String),
    
    /// Failed to deserialize data from bytes format.
    ///
    /// This variant is used when binary data cannot be converted back to
    /// data structures due to format issues or data corruption.
    ///
    /// Common cases include:
    /// - Truncated or incomplete data
    /// - Version mismatches between serializer/deserializer
    /// - Corrupted binary data
    /// - Invalid data structure relationships
    UnableToDeserializeBytes(String),
    
    /// Incompatible usage of bytes structures.
    ///
    /// This variant is used when bytes structures are used in contexts they
    /// weren't designed for, or when operations are performed on incompatible formats.
    ///
    /// Common cases include:
    /// - Using sensor bytes format for motor data
    /// - Mixing different protocol versions
    /// - Operating on wrong endianness
    IncompatibleByteUse(String),
}

impl fmt::Display for FeagiBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            FeagiBytesError::UnableToValidateBytes(e) => write!(f, "Unable to validate bytes: {}", e),
            FeagiBytesError::UnableToSerializeBytes(e) => write!(f, "Unable to serialize bytes: {}", e),
            FeagiBytesError::UnableToDeserializeBytes(e) => write!(f, "Unable to deserialize bytes: {}", e),
            FeagiBytesError::IncompatibleByteUse(e) => write!(f, "Incorrect of bytes structure: {}", e),
        }
    }
}

/// Errors related to hardware devices and I/O operations.
///
/// This error type covers issues that occur during interaction with hardware devices,
/// including sensor input validation, motor output processing, and device communication.
/// Essential for embodiment systems and hardware integration.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::IODeviceError;
///
/// // Sensor error
/// let error = IODeviceError::InvalidSensorInputValues("Camera disconnected".to_string());
///
/// // Motor error
/// let error = IODeviceError::InvalidMotorCallback("Motor callback returned null".to_string());
/// ```
#[derive(Debug)]
pub enum IODeviceError {
    /// Invalid sensor input values or sensor failures.
    ///
    /// This variant is used when sensor data is outside expected ranges,
    /// sensors fail to provide data, or sensor configurations are invalid.
    ///
    /// Common cases include:
    /// - Sensor values outside calibrated ranges
    /// - Sensor hardware disconnection or failure
    /// - Invalid sensor configuration parameters
    /// - Corrupt sensor data transmission
    InvalidSensorInputValues(String),
    
    /// Invalid motor callback function or motor control errors.
    ///
    /// This variant is used when motor callback functions fail to execute
    /// properly or return invalid results.
    ///
    /// Common cases include:
    /// - Motor callback function returns errors
    /// - Invalid motor control parameters
    /// - Motor hardware communication failures
    /// - Callback function not properly registered
    InvalidMotorCallback(String),
    
    /// Invalid motor data received from FEAGI system.
    ///
    /// This variant is used when the FEAGI system sends motor control data
    /// that cannot be processed or is outside valid parameters.
    ///
    /// Common cases include:
    /// - Motor commands outside hardware limits
    /// - Invalid motor control protocol data
    /// - Corrupted motor command transmission
    /// - Unsupported motor control operations
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

/// Errors related to neuron data processing and format conversion.
///
/// This error type covers issues that occur during neuron data operations, including
/// parsing neuron data structures, converting between neuron formats, and generating
/// neuron data for brain state processing.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::NeuronError;
///
/// // Parsing error
/// let error = NeuronError::UnableToParseFromNeuronData("Invalid XYZP coordinates".to_string());
///
/// // Conversion error
/// let error = NeuronError::UnableToConvertNeuronFormat("Incompatible neuron formats".to_string());
/// ```
#[derive(Debug)]
pub enum NeuronError {
    /// Failed to parse neuron data from input format.
    ///
    /// This variant is used when neuron data cannot be parsed from its
    /// source format due to structural issues or invalid data.
    ///
    /// Common cases include:
    /// - Invalid XYZP coordinate data
    /// - Corrupted neuron state information
    /// - Missing required neuron metadata
    /// - Invalid cortical mapping data
    UnableToParseFromNeuronData(String),
    
    /// Failed to convert between neuron data formats.
    ///
    /// This variant is used when neuron data cannot be converted from
    /// one format to another due to incompatibility or data loss.
    ///
    /// Common cases include:
    /// - Incompatible neuron encoding formats
    /// - Loss of precision during format conversion
    /// - Unsupported target neuron format
    /// - Missing conversion metadata
    UnableToConvertNeuronFormat(String),
    
    /// Failed to generate neuron data structures.
    ///
    /// This variant is used when neuron data cannot be generated or
    /// constructed due to invalid parameters or system constraints.
    ///
    /// Common cases include:
    /// - Invalid neuron generation parameters
    /// - Insufficient system resources for neuron data
    /// - Invalid cortical area specifications
    /// - Conflicts in neuron data requirements
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

/// Errors related to genome structure and cortical area processing.
///
/// This error type covers issues that occur during genome processing, including
/// cortical ID validation, dimension checking, and channel configuration validation.
/// Critical for brain architecture validation and cortical area management.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::GenomeError;
///
/// // Cortical ID error
/// let error = GenomeError::InvalidCorticalID("Cortical ID out of range".to_string());
///
/// // Dimension error
/// let error = GenomeError::InvalidCorticalDimensions("Negative dimensions not allowed".to_string());
/// ```
#[derive(Debug)]
pub enum GenomeError {
    /// Invalid cortical area identifier.
    ///
    /// This variant is used when cortical IDs are outside valid ranges,
    /// reference non-existent cortical areas, or have invalid formats.
    ///
    /// Common cases include:
    /// - Cortical ID outside valid numeric range
    /// - Reference to unregistered cortical area
    /// - Invalid cortical ID encoding format
    /// - Cortical ID conflicts in brain architecture
    InvalidCorticalID(String),
    
    /// Invalid cortical area dimensions.
    ///
    /// This variant is used when cortical area dimensions are invalid,
    /// outside system limits, or incompatible with brain architecture.
    ///
    /// Common cases include:
    /// - Negative or zero cortical dimensions
    /// - Dimensions exceeding system memory limits
    /// - Incompatible dimension ratios
    /// - Dimension conflicts with existing cortical areas
    InvalidCorticalDimensions(String),
    
    /// Invalid channel dimensions or configuration.
    ///
    /// This variant is used when channel configurations are invalid,
    /// exceed cortical area capacity, or have incompatible parameters.
    ///
    /// Common cases include:
    /// - Channel count exceeding cortical area limits
    /// - Invalid channel index mappings
    /// - Incompatible channel data types
    /// - Channel configuration conflicts
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

/// Errors related to command and control protocol operations.
///
/// This error type covers issues that occur during command/control communication,
/// including parameter validation, response processing, authentication, and
/// protocol compliance. Essential for system coordination and security.
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::error::CommandAndControlError;
///
/// // Command error
/// let error = CommandAndControlError::InvalidCommandParameter("Unknown command type".to_string());
///
/// // Authentication error
/// let error = CommandAndControlError::AuthenticationFailure("Invalid credentials".to_string());
/// ```
#[derive(Debug)]
pub enum CommandAndControlError {
    /// Invalid command parameter or command structure.
    ///
    /// This variant is used when command parameters are outside valid ranges,
    /// have invalid formats, or reference non-existent system basic_components.
    ///
    /// Common cases include:
    /// - Unknown command types or operations
    /// - Command parameters outside valid ranges
    /// - Invalid command syntax or structure
    /// - Missing required command parameters
    InvalidCommandParameter(String),
    
    /// Invalid response received from command execution.
    ///
    /// This variant is used when command responses have invalid formats,
    /// unexpected content, or indicate command execution failures.
    ///
    /// Common cases include:
    /// - Response format doesn't match protocol
    /// - Unexpected response content or status
    /// - Response indicates command execution failure
    /// - Corrupted response data
    InvalidResponse(String),
    
    /// No response received for command.
    ///
    /// This variant is used when commands are sent but no response is
    /// received within expected timeouts or communication fails.
    ///
    /// Common cases include:
    /// - Command timeout without response
    /// - Communication channel failure
    /// - System component not responding
    /// - Network or connection issues
    NoResponse(String),
    
    /// Authentication or authorization failure.
    ///
    /// This variant is used when authentication credentials are invalid,
    /// authorization is denied, or security protocols are violated.
    ///
    /// Common cases include:
    /// - Invalid username/password credentials
    /// - Expired authentication tokens
    /// - Insufficient permissions for operation
    /// - Security protocol violations
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