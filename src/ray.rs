use derive_new::new as New;

use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, New, Default)]
pub struct Ray {
    pub origin: Point3, // point
    pub dir: Vec3,      // vector
    pub time: f64,
}

impl Ray {
    pub fn new_no_time(origin: Point3, dir: Vec3) -> Ray {
        Ray {
            origin,
            dir,
            time: 0.0,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.dir * t)
    }
}
