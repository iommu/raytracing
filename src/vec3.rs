use std::ops::{self, Index, IndexMut};

use crate::utils::{random_double, random_double_range};

extern crate nalgebra as na;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(na::Vector3<f64>);

impl Vec3 {
    // generators
    #[allow(dead_code)]
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(na::Vector3::new(x, y, z))
    }

    #[allow(dead_code)]
    #[inline]
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    #[allow(dead_code)]
    #[inline]
    pub fn random_from_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    #[allow(dead_code)]
    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let point = Self::random_from_range(-1.0, 1.0);
            let len_s_point = point.len_squared();

            if 1e-160 < len_s_point && len_s_point <= 1.0 {
                return point / len_s_point;
            }
        }
    }

    // mapped primatives

    #[allow(dead_code)]
    #[inline]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[allow(dead_code)]
    #[inline]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[allow(dead_code)]
    #[inline]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    #[allow(dead_code)]
    #[inline]
    pub fn len(&self) -> f64 {
        self.0.magnitude()
    }

    #[allow(dead_code)]
    #[inline]
    pub fn len_squared(&self) -> f64 {
        self.0.magnitude_squared()
    }

    // map generators

    #[allow(dead_code)]
    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.0.dot(&rhs.0)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0.cross(&rhs.0))
    }

    // member funcs

    #[allow(dead_code)]
    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.len()
    }

    #[allow(dead_code)]
    #[inline]
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - &(*n * (self.dot(&n) * 2.0))
    }

    #[allow(dead_code)]
    #[inline]
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((*self * -1.0).dot(n), 1.0);
        let ray_out_perp = (*self + &(*n * cos_theta)) * etai_over_etat;
        let ray_out_para = *n * ((1.0 - ray_out_perp.len_squared()).abs().sqrt() * -1.0);
        ray_out_perp + &ray_out_para
    }

    #[allow(dead_code)]
    #[inline]
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions
        let thresh = 1e-8;
        return (self.x().abs() < thresh) && (self.y().abs() < thresh) && (self.z().abs() < thresh);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(self) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }

    // static functions

    #[allow(dead_code)]
    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec3) {
        self.0.add_assign(&rhs.0)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f64) {
        self.0.mul_assign(t)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: f64) {
        self.0.div_assign(t)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: f64) -> Vec3 {
        Vec3(self.0.add_scalar(rhs))
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Vec3 {
        Vec3(self.0 / rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs)
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0.component_mul(&rhs.0))
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3(-self.0)
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Aliases
pub type Color = Vec3;
pub type Point3 = Vec3;
