use std::rc::Rc;

use derive_new::new as New;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, New, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[allow(dead_code)]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length

        self.front_face = Vec3::dot(&ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    #[allow(dead_code)]
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                temp_rec.clone_into(rec);
            }
        }

        hit_anything
    }
}
