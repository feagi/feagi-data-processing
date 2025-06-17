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

/// Represents the dimensions of a cortical area. All dimension directions must be non-zero
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CorticalDimensions {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl CorticalDimensions {
    pub fn new(x: usize, y: usize, z: usize) -> Result<CorticalDimensions, DataProcessingError> {
        if x == 0 || y == 0 || z == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Cortical dimensions cannot be 0 in any direction!".into()));
        }
        Ok(CorticalDimensions { x, y, z })
    }

    pub fn as_tuple(&self) -> (usize, usize, usize) {
        (self.x, self.y, self.z)
    }

    pub fn verify(&self) -> Result<(), DataProcessingError> {
        if self.x == 0 || self.y == 0 || self.z == 0 {
            return Err(DataProcessingError::InvalidInputBounds("Cortical dimensions cannot be 0 in any direction!".into()));
        }
        Ok(())
    }

}

