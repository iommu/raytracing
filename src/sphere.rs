use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat : &Option<Rc<dyn Material>>) -> Sphere {
        Sphere {
            center: center.clone(),
            radius: radius,
            mat: mat.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = &self.center - ray.origin();
        let a = ray.direction().len_squared();
        let h = Vec3::dot(ray.direction(), &oc);
        let c = oc.len_squared() - self.radius * self.radius;
        //
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = &(&rec.p - &self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.normal = &(&rec.p - &self.center) / self.radius;
        rec.mat = self.mat.clone();

        return true;
    }
}
