use super::Lerp;

impl Lerp for f32 {
    fn lerp_clamped(&self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.clamp(0.0, 1.0)
    }
}

impl Lerp for f64 {
    fn lerp_clamped(&self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.clamp(0.0, 1.0)
    }
}
