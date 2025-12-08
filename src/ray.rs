use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    origin: Vec3, // point
    dir: Vec3,    // vector
}

impl Ray {
    fn new(origin: &Vec3, dir: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            dir: dir.clone(),
        }
    }

    fn origin(&self) -> &Vec3 {
        &self.origin
    }

    fn direction(&self) -> &Vec3 {
        &self.dir
    }

    fn at(&self, t : f64) -> Vec3 {
        &self.origin + &(&self.dir*t)
    }
}
