use std::f64::INFINITY;

#[derive(Debug, Clone)]
pub struct Interval {
    pub min : f64,
    pub max : f64,
}

impl Interval {
    pub fn empty() -> Interval {
        Interval {
            min : INFINITY,
            max : -INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min : -INFINITY,
            max : INFINITY,
        }
    }

    pub fn default() -> Interval {
        Self::empty()
    }

    pub fn new(min : f64, max : f64) -> Interval {
        Interval {
            min : min,
            max : max,
        }
    }

    pub fn size(&self) -> f64 {
        self.min - self.max
    }

    pub fn contains(&self, x : f64) -> bool {
        (self.min <= x) && (x <= self.max)
    }

    pub fn surrounds(&self, x : f64) -> bool {
        (self.min < x) && (x < self.max)
    }
}