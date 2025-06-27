use crate::error::IODataError;

const EPSILON: f32 = 0.00001;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LinearBoundedF32 {
    value: f32,
    upper: f32,
    lower: f32,
}

impl std::fmt::Display for LinearBoundedF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Linear Bounded Value: Lower Bound: {}, Upper Bound: {}, Value: {}", self.lower, self.upper, self.value)
    }
}

impl LinearBoundedF32 {

    pub fn new(value: f32, upper_bound: f32, lower_bound: f32) -> Result<Self, IODataError> {
        Self::validate_bounds(upper_bound, lower_bound)?;
        validate_float_is_real(value)?;
        Self::validate_value_bounds(value, upper_bound, lower_bound)?;
        Ok(LinearBoundedF32 {
            value,
            upper: upper_bound,
            lower: lower_bound,
        })
    }

    pub fn new_with_clamp(value: f32, upper_bound: f32, lower_bound: f32) -> Result<Self, IODataError> {
        Self::validate_bounds(upper_bound, lower_bound)?;
        validate_float_is_real(value)?;
        let final_val = value.clamp(lower_bound, upper_bound);

        Ok(LinearBoundedF32 {
            value: final_val,
            upper: upper_bound,
            lower: lower_bound,
        })
    }

    pub fn new_from_normalized(value: LinearNormalizedF32, upper_bound: f32, lower_bound: f32) -> Result<Self, IODataError> {
        Self::validate_bounds(upper_bound, lower_bound)?;
        let avg = calculate_avg(upper_bound, lower_bound);
        let half_distance = calculate_half_distance(upper_bound, lower_bound);

        Ok(LinearBoundedF32 {
            value: scale_from_normalized(value.asf32(), avg, half_distance),
            upper: upper_bound,
            lower: lower_bound,
        })
    }

    pub fn as_nonnormalized_float(&self) -> f32 {
        self.value
    }

    pub fn as_normalized_f32(&self) -> LinearNormalizedF32 {
        let avg = calculate_avg(self.upper, self.lower);
        let half_distance = calculate_half_distance(self.upper, self.lower);
        LinearNormalizedF32 {
            float: scale_to_normalized(self.value, avg, half_distance), // no need to do checks
        }
    }

    pub fn get_upper_lower_bounds(&self) -> (f32, f32) {
        (self.upper, self.lower)
    }

    pub fn update_value(&mut self, new_value: f32) -> Result<(), IODataError> {
        validate_float_is_real(new_value)?;
        Self::validate_value_bounds(new_value, self.upper, self.lower)?;
        self.value = new_value;
        Ok(())
    }

    pub fn update_bounds(&mut self, upper_bound: f32, lower_bound: f32) -> Result<(), IODataError> {
        Self::validate_bounds( upper_bound, lower_bound)?;
        if self.value > upper_bound {
            return Err(IODataError::InvalidParameters("Upper bound may not be smaller than the current value!".into()));
        }
        if self.value < lower_bound {
            return Err(IODataError::InvalidParameters("Lower bound may not be larger than the current value!".into()));
        }
        self.upper = upper_bound;
        self.lower = lower_bound;
        Ok(())
    }


    fn validate_value_bounds(value: f32, upper: f32, lower: f32)  -> Result<(), IODataError> {
        if value > upper {
            return Err(IODataError::InvalidParameters("Value may not be Greater than upper bound!".into()));
        }
        if value < lower {
            return Err(IODataError::InvalidParameters("Value may not be Less than lower bound!".into()));
        }
        Ok(())
    }



    fn validate_bounds(upper: f32, lower: f32) -> Result<(), IODataError> {
        validate_float_is_real(upper)?;
        validate_float_is_real(lower)?;
        if lower >= upper {
            return Err(IODataError::InvalidParameters("Lower bound may not be greater than the upper!".into()));
        }
        Ok(())
    }

}


#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
pub struct LinearNormalizedF32 {
    float: f32
}

impl std::fmt::Display for LinearNormalizedF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        write!(f, "Linear -1 <-> 1 Bounded Value: {}", self.float)
    }
}

impl LinearNormalizedF32 {
    pub fn new(float: f32) -> Result<Self, IODataError> {
        validate_float_is_real(float)?;
        LinearNormalizedF32::validate_float_range(float)?;
        Ok(Self { float })
    }

    pub fn new_with_clamp(float: f32)  -> Result<Self, IODataError> {
        if float.is_nan() {
            return Err(IODataError::InvalidParameters("Input float may not be NaN!".into()));
        }
        let clamped = float.clamp(-1.0, 1.0);
        Ok(Self { float: clamped })
    }

    pub fn new_from_bound(linear_bounded_f32: LinearBoundedF32) -> Self {
        let upper = linear_bounded_f32.upper;
        let lower = linear_bounded_f32.lower;
        let avg = calculate_avg(upper, lower);
        let half_distance = calculate_half_distance(upper, lower);
        Self { float:  scale_to_normalized(linear_bounded_f32.value, avg, half_distance)}
    }

    pub fn new_zero() -> Self {
        Self { float: 0.0 }
    }

    pub fn update(&mut self, new_float: f32) -> Result<(), IODataError> {
        validate_float_is_real(new_float)?;
        LinearNormalizedF32::validate_float_range(new_float)?;
        self.float = new_float;
        Ok(())
    }

    pub fn update_with_clamp(&mut self, new_float: f32) -> Result<(), IODataError> {
        if new_float.is_nan() {
            return Err(IODataError::InvalidParameters("Input float may not be NaN!".into()));
        }
        self.float = new_float.clamp(-1.0 - EPSILON, 1.0 + EPSILON);
        Ok(())
    }

    pub fn asf32(&self) -> f32 {
        self.float
    }

    pub fn is_sign_positive(&self) -> bool {
        self.float.is_sign_positive()
    }

    fn validate_float_range(float: f32) -> Result<(), IODataError> {
        if float.abs() > 1.0 + EPSILON {
            return Err(IODataError::InvalidParameters("Input float may not be less than negative one or greater than 1!".into()));
        }
        Ok(())
    }
}

fn validate_float_is_real(value: f32) -> Result<(), IODataError> {
    if value.is_nan() {
        return Err(IODataError::InvalidParameters("Value may not be NaN!".into()));
    }
    if value.is_infinite() {
        return Err(IODataError::InvalidParameters("Value may not be Infinite!".into()));
    }
    Ok(())
}

fn scale_from_normalized(normalized_value: f32, average: f32, half_distance: f32) -> f32 {
    normalized_value * half_distance + average
}

fn scale_to_normalized(nonnormalized_value: f32, average: f32, half_distance: f32) -> f32 {
    (nonnormalized_value - average) / half_distance
}

fn calculate_avg(upper_bound: f32, lower_bound: f32) -> f32 {
    (lower_bound + upper_bound) / 2.0
}

fn calculate_half_distance(upper_bound: f32, lower_bound: f32) -> f32 {
    (upper_bound - lower_bound) / 2.0
}