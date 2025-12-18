use std::{f64::consts::PI, rc::Rc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn get_uv(point: Point3, u: &mut f64, v: &mut f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-(point.y)).acos();
        let phi = f64::atan2(-(point.z), point.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
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
        Self::get_uv(outward_normal, &mut rec.u, &mut rec.v);
        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
