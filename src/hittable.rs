use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct HitRecord {
    pub p : Point3,
    pub normal : Vec3,
    pub t : f64
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, ray_tmin : f64, ray_tmax : f64, rec : &mut HitRecord) -> bool;
}