use rand::Rng;
use std::f64::consts::PI;

#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
#[inline(always)]
pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}
#[inline(always)]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
