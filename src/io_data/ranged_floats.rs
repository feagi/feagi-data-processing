use crate::error::{IODataError, FeagiDataProcessingError};
use std::convert::{Into};

const EPSILON: f32 = 0.00001;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BoundedF32 {
    value: f32,
    upper: f32,
    lower: f32,
}

impl std::fmt::Display for BoundedF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Linear Bounded Value: Lower Bound: {}, Upper Bound: {}, Value: {}", self.lower, self.upper, self.value)
    }
}

impl Into<f32> for BoundedF32 {
    fn into(self) -> f32 {
        self.value
    }
}

impl PartialOrd for BoundedF32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl BoundedF32 {

    pub fn new_with_error(value: f32, lower_bound: f32, upper_bound: f32) -> Result<Self, FeagiDataProcessingError> {
        verify_if_float_real(value)?;
        verify_if_float_real(upper_bound)?;
        verify_if_float_real(lower_bound)?;
        verify_bounds_ordered(lower_bound, upper_bound)?;
        verify_in_bounds(value, lower_bound, upper_bound)?;
        Ok(BoundedF32 {
            value,
            upper: upper_bound,
            lower: lower_bound,
        })
    }

    pub fn new_with_clamp(value: f32, upper_bound: f32, lower_bound: f32) -> Result<Self, FeagiDataProcessingError> {
        verify_if_float_real(value)?;
        verify_if_float_real(upper_bound)?;
        verify_if_float_real(lower_bound)?;
        verify_bounds_ordered(lower_bound, upper_bound)?;
        let value = value.clamp(lower_bound, upper_bound);

        Ok(BoundedF32 {
            value,
            upper: upper_bound,
            lower: lower_bound,
        })
    }
    

    pub fn update_value_with_error(&mut self, new_value: f32) -> Result<(), FeagiDataProcessingError> {
        verify_if_float_real(new_value)?;
        verify_in_bounds(new_value, self.lower, self.upper)?;
        self.value = new_value;
        Ok(())
    }

    pub fn update_value_with_clamp(&mut self, new_value: f32) -> Result<(), FeagiDataProcessingError> {
        verify_if_float_real(new_value)?;
        let new_value = new_value.clamp(self.lower, self.upper);
        self.value = new_value;
        Ok(())
    }

    pub fn update_bounds(&mut self, lower_bound: f32, upper_bound: f32) -> Result<(), FeagiDataProcessingError> {
        verify_if_float_real(lower_bound)?;
        verify_if_float_real(upper_bound)?;
        verify_bounds_ordered(lower_bound, upper_bound)?;
        self.upper = upper_bound;
        self.lower = lower_bound;
        Ok(())
    }
}


// TODO change the ranges here to constants


#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
pub struct NormalizedM1To1F32 {
    value: BoundedF32
}

impl std::fmt::Display for NormalizedM1To1F32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (LinearM1to1NormalizedF32)", self.value)
    }
}

impl Into<f32> for NormalizedM1To1F32 {
    fn into(self) -> f32 {
        self.value.value
    }
}

impl NormalizedM1To1F32 {
    pub fn new(float: f32) -> Result<Self, FeagiDataProcessingError> {
        Ok(Self { value: BoundedF32::new_with_error(float, -1.0, 1.0)? })
    }

    pub fn new_with_clamp(float: f32)  -> Result<Self, FeagiDataProcessingError> {
        Ok(Self { value: BoundedF32::new_with_clamp(float, -1.0, 1.0)? })
    }
    pub fn new_zero() -> Self {
        Self {
            value: BoundedF32{value: 0.0, upper: 1.0, lower: -1.0}
        }
    }

    pub fn update_value_with_error(&mut self, new_float: f32) -> Result<(), FeagiDataProcessingError> {
        self.value.update_value_with_error(new_float)?;
        Ok(())
    }

    pub fn update_value_with_clamp(&mut self, new_float: f32) -> Result<(), FeagiDataProcessingError> {
        self.value.update_value_with_clamp(new_float)?;
        Ok(())
    }
    
    pub fn is_sign_positive(&self) -> bool {
        self.value.value.is_sign_positive()
    }
}


#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
pub struct Normalized0To1F32 {
    value: BoundedF32
}

impl std::fmt::Display for Normalized0To1F32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        write!(f, "{} (Linear0to1NormalizedF32)", self.value)
    }
}

impl Into<f32> for Normalized0To1F32 {
    fn into(self) -> f32 {
        self.value.value
    }
}

impl Normalized0To1F32 {
    pub fn new(float: f32) -> Result<Self, FeagiDataProcessingError> {
        Ok(Self { value: BoundedF32::new_with_error(float, 0.0, 1.0)? })
    }

    pub fn new_with_clamp(float: f32)  -> Result<Self, FeagiDataProcessingError> {
        Ok(Self { value: BoundedF32::new_with_clamp(float, 0.0, 1.0)? })
    }
    pub fn new_zero() -> Self {
        Self {
            value: BoundedF32{value: 0.0, upper: 1.0, lower: 0.0}
        }
    }

    pub fn update_value_with_error(&mut self, new_float: f32) -> Result<(), FeagiDataProcessingError> {
        self.value.update_value_with_error(new_float)?;
        Ok(())
    }

    pub fn update_value_with_clamp(&mut self, new_float: f32) -> Result<(), FeagiDataProcessingError> {
        self.value.update_value_with_clamp(new_float)?;
        Ok(())
    }
}


fn verify_if_float_real(value: f32) -> Result<(), FeagiDataProcessingError> {
    if value.is_nan() {
        return Err(IODataError::InvalidParameters("Value may not be NaN!".into()).into());
    }
    if value.is_infinite() {
        return Err(IODataError::InvalidParameters("Value may not be Infinite!".into()).into());
    }
    Ok(())
}

fn verify_bounds_ordered(lower: f32, upper: f32) -> Result<(), FeagiDataProcessingError> {
    if lower >= upper {
        return Err(IODataError::InvalidParameters(format!("Lower bound {} may not be greater than the upper bound {}!", lower, upper)).into())
    }
    Ok(())
}

fn verify_in_bounds(value: f32, lower_bound: f32, upper_bound: f32) -> Result<(), FeagiDataProcessingError> {
    if value >= lower_bound && value <= upper_bound {
        return Ok(());
    }
    Err(IODataError::InvalidParameters(format!("Value {} is not within bounds {} and {}!", value, lower_bound, upper_bound)).into())
}