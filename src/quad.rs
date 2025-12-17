use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{self, HitRecord, Hittable},
    interval::{self, Interval},
    material::Material,
    ray,
    vec3::{Point3, Vec3},
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    #[allow(dead_code)]
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        Self {
            q,
            u,
            v,
            w: n / n.dot(n),
            mat,
            bbox: Self::compute_bbox(q, u, v),
            normal,
            d: normal.dot(q),
        }
    }

    #[allow(dead_code)]
    fn compute_bbox(q: Point3, u: Vec3, v: Vec3) -> AABB {
        // Compute the bounding box of all four vertices
        let bbox_diag_1 = AABB::from_points(q, q + u + v);
        let bbox_diag_2 = AABB::from_points(q + u, q + v);
        AABB::from_aabbs(bbox_diag_1, bbox_diag_2)
    }

    #[allow(dead_code)]
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = a;
        return true;
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &ray::Ray,
        ray_t: interval::Interval,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let denom = self.normal.dot(ray.dir);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval
        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates
        let intersection = ray.at(t);
        let planar_hitpt_vec = intersection - self.q;
        let beta = self.w.dot(planar_hitpt_vec.cross(self.v));
        let alpha = self.w.dot(self.u.cross(planar_hitpt_vec));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true
        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(ray, self.normal);

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
