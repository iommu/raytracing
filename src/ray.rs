use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3, // point
    dir: Vec3,    // vector
}

impl Ray {
    pub fn new(origin: &Vec3, dir: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            dir: dir.clone(),
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t : f64) -> Vec3 {
        &self.origin + &(&self.dir*t)
    }
}
