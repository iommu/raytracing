use std::{cmp::Ordering, rc::Rc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    utils::random_int_range,
};

#[derive(Clone)]
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_list(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }

    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects
        let mut bbox = AABB::empty();
        for object in objects.iter() {
            bbox = AABB::from_aabbs(bbox, object.bounding_box());
        }

        let axis = random_int_range(0, 2) as usize;

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            (objects[start].clone(), objects[start + 1].clone())
        } else {
            objects.sort_by(|a, b| Self::box_compare(a, b, axis));
            let mid = start + object_span / 2;
            (
                Rc::new(Self::new(objects, start, mid)) as Rc<dyn Hittable>,
                Rc::new(Self::new(objects, mid, end)) as Rc<dyn Hittable>,
            )
        };

        Self {
            left: left,
            right: right,
            bbox: bbox,
        }
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box()[axis_index];
        let b_axis_interval = b.bounding_box()[axis_index];
        return if a_axis_interval.min < b_axis_interval.min {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, rec);
        let hit_right = self.right.hit(ray, Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
