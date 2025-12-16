use std::f64::INFINITY;

use derive_new::new as New;

#[derive(Debug, Clone, Copy, New)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    #[allow(dead_code)]
    pub fn empty() -> Interval {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    #[allow(dead_code)]
    pub fn universe() -> Interval {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    #[allow(dead_code)]
    pub fn from_intervals(a: Interval, b: Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> f64 {
        self.min - self.max
    }

    #[allow(dead_code)]
    pub fn contains(&self, x: f64) -> bool {
        (self.min <= x) && (x <= self.max)
    }

    #[allow(dead_code)]
    pub fn surrounds(&self, x: f64) -> bool {
        (self.min < x) && (x < self.max)
    }

    #[allow(dead_code)]
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    #[allow(dead_code)]
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}
