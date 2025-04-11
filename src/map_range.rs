use std::ops::{Div, Sub};
mod implementations;
mod tests;

use crate::{error::ScalarError, lerp::Lerp};

pub trait MapRange: Sized + PartialEq + Sub<Output = Self> + Div<Output = Self> + Lerp {
    fn map_range_unclamped(&self, in_min: Self, in_max: Self, out_min: Self, out_max: Self) -> Result<Self, ScalarError> {
        if in_min == in_max {
            return Err(ScalarError::InvalidRangeError(format!("Input range cannot be zero")));
        }

        let t = (*self - in_min) / (in_max - in_min);
        Ok(t.lerp_unclamped(out_min, out_max))
    }
    fn map_range_clamped(&self, in_min: &Self, in_max: &Self, out_min: &Self, out_max: &Self) -> Result<Self, ScalarError> {
        if in_min == in_max {
            return Err(ScalarError::InvalidRangeError(format!("Input range cannot be zero")));
        }
        let t = (*self - *in_min) / (*in_max - *in_min);
        Ok(t.lerp_clamped(*out_min, *out_max))
    }
}
