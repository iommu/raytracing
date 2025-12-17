use lazy_static::lazy_static;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::f64::consts::PI;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<SmallRng> = {
        // Initialize StdRng using a system-provided seed (cryptographically secure)
        Mutex::new(SmallRng::from_rng(&mut rand::rng()))
    };
}

#[allow(dead_code)]
#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    RNG.lock()
        .expect("Failed to lock the global RNG Mutex")
        .random_range(0.0..1.0)
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    RNG.lock()
        .expect("Failed to lock the global RNG Mutex")
        .random_range(min..max)
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_int_range(min: i32, max: i32) -> i32 {
    let max = max + 1;
    // Returns a random real in [min,max+1).
    RNG.lock()
        .expect("Failed to lock the global RNG Mutex")
        .random_range(min..max)
}

#[allow(dead_code)]
#[inline(always)]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
