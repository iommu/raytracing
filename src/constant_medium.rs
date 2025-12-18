use std::{f64::{INFINITY, consts::E}, rc::Rc};

use crate::{
    hittable::{self, HitRecord, Hittable}, interval::{self, Interval}, material::{Isotropic, Material}, ray, texture::Texture, utils::random_double, vec3::{Color, Vec3}
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, tex: Rc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Rc::new(Isotropic::new(tex)),
        }
    }

    pub fn from_color(boundary: Rc<dyn Hittable>, density: f64, albedo: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Rc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &ray::Ray, ray_t: interval::Interval, rec: &mut hittable::HitRecord) -> bool {
        let mut rec_1 = HitRecord::default();
        let mut rec_2 = HitRecord::default();

        if !self.boundary.hit(ray, Interval::universe(), &mut rec_1) {
            return false;
        }

        if !self.boundary.hit(ray, Interval::new(rec_1.t+0.001, INFINITY), &mut rec_2) {
            return false;
        }

        if rec_1.t < ray_t.min {
            rec_1.t = ray_t.min;
        }
        if rec_2.t > ray_t.max {
            rec_2.t = ray_t.max;
        }

        if rec_1.t >= rec_2.t {
            return false;
        }

        if rec_1.t < 0.0 {
            rec_1.t = 0.0;
        }

        let ray_len = ray.dir.len();
        let dist_inside_boundary = (rec_2.t - rec_1.t) * ray_len;
        let hit_dist = self.neg_inv_density * random_double().log(E);

        if hit_dist > dist_inside_boundary {
            return false;
        }

        rec.t = rec_1.t + hit_dist / ray_len;
        rec.p = ray.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitary
        rec.front_face = true; // also arbitary
        rec.mat = Some(self.phase_function.clone());
    

        return  true;
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
