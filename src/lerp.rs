use std::ops::{Add, Mul, Sub};
mod implementations;
mod tests;

pub trait Lerp: Copy + Sub<Output = Self> + Mul<Output = Self> + Add<Output = Self> {
    /// Linearly interpolates between two values `a` and `b` using `self`.
    ///
    /// Values of `self` outside the range `[0, 1]` will result in extrapolation.
    fn lerp_unclamped(&self, a: Self, b: Self) -> Self {
        a + (b - a) * *self
    }

    /// Linearly interpolates between two values `a` and `b` using `self`.
    ///
    /// Values of `self` outside the range `[0, 1]` will be clamped.
    fn lerp_clamped(&self, a: Self, b: Self) -> Self;
}
