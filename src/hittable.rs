use std::{f64::INFINITY, rc::Rc};

use derive_new::new as New;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};

#[derive(Clone, New, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[allow(dead_code)]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length

        self.front_face = Vec3::dot(&ray.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut list = Self::default();
        list.add(object);
        list
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    #[allow(dead_code)]
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::from_aabbs(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox: bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(ray.origin - &self.offset, ray.dir, ray.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += &self.offset;

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        let bbox = AABB::default();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let cos_theta = self.cos_theta;
        let sin_theta = self.sin_theta;
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            (cos_theta * ray.origin.x()) - (sin_theta * ray.origin.z()),
            ray.origin.y(),
            (sin_theta * ray.origin.x()) + (cos_theta * ray.origin.z()),
        );

        let dir = Vec3::new(
            (cos_theta * ray.dir.x()) - (sin_theta * ray.dir.z()),
            ray.dir.y(),
            (sin_theta * ray.dir.x()) + (cos_theta * ray.dir.z()),
        );

        let rotated_r = Ray::new(origin, dir, ray.time);

        // Determine whether an intersection exists in object space (and if so, where).
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space
        rec.p = Point3::new(
            (cos_theta * rec.p.x()) + (sin_theta * rec.p.z()),
            rec.p.y(),
            (-sin_theta * rec.p.x()) + (cos_theta * rec.p.z()),
        );

        rec.normal = Vec3::new(
            (cos_theta * rec.normal.x()) + (sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-sin_theta * rec.normal.x()) + (cos_theta * rec.normal.z()),
        );

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
