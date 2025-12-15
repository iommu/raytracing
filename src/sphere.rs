use std::rc::Rc;

use derive_new::new as New;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Dielectric, Lambertian, Material},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new_stationary(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let radius = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new_no_time(static_center, Vec3::default()),
            radius: radius,
            mat: mat,
            bbox: AABB::from_points(static_center - rvec, static_center + rvec),
        }
    }

    pub fn new_moving(
        center_1: Point3,
        center_2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Sphere {
        let radius = radius.max(0.0);
        let center = Ray::new_no_time(center_1, center_2 - center_1);
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: center,
            radius: radius,
            mat: mat,
            bbox: AABB::from_aabbs(
                AABB::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec),
                AABB::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec),
            ),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
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
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        
        rec.mat = Some(self.mat.clone());

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
