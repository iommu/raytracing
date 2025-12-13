use derive_new::new as New;

use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, New, Default)]
pub struct Ray {
    origin: Point3, // point
    dir: Vec3,      // vector
}

impl Ray {
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.dir * t)
    }
}
