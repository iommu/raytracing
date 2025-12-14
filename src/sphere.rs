use std::rc::Rc;

use derive_new::new as New;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, New, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.dir.len_squared();
        let h = Vec3::dot(&ray.dir, oc);
        let c = oc.len_squared() - self.radius * self.radius;
        //
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
