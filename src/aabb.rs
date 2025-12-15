use std::ops::Index;

use derive_new::new as New;

use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Debug, Clone, Copy, Default, New)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    #[allow(dead_code)]
    pub fn empty() -> AABB {
        AABB {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    #[allow(dead_code)]
    pub fn universe() -> AABB {
        AABB {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    #[allow(dead_code)]
    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: if a[0] <= b[0] {
                Interval::new(a[0], b[0])
            } else {
                Interval::new(b[0], a[0])
            },
            y: if a[1] <= b[1] {
                Interval::new(a[1], b[1])
            } else {
                Interval::new(b[1], a[1])
            },
            z: if a[2] <= b[2] {
                Interval::new(a[2], b[2])
            } else {
                Interval::new(b[2], a[2])
            },
        }
    }

    #[allow(dead_code)]
    pub fn from_aabbs(box_0: AABB, box_1: AABB) -> Self {
        Self {
            x: Interval::from_intervals(box_0.x, box_1.x),
            y: Interval::from_intervals(box_0.y, box_1.y),
            z: Interval::from_intervals(box_0.z, box_1.z),
        }
    }

    #[allow(dead_code)]
    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = &ray.origin;
        let ray_dir = &ray.dir;

        for axis in 0usize..3 {
            let ax = self[axis];
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box

        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            } else {
                return 2;
            }
        }
    }
}

impl Index<usize> for AABB {
    type Output = Interval;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3, "Index out of bounds!"); // Bounds check
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}
