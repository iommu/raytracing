use std::ops;

use crate::utils::{random_double, random_double_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    #[allow(dead_code)]
    pub fn default() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    #[allow(dead_code)]
    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    #[allow(dead_code)]
    pub fn random_from_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    #[allow(dead_code)]
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let point = Vec3::random_from_range(-1.0, 1.0);
            let len_s_point = point.len_squared();
            if 1e-160 < len_s_point && len_s_point <= 1.0 {
                return point / len_s_point;
            }
        }
    }

    #[allow(dead_code)]
    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, *self) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - (n * (Vec3::dot(self, n) * 2.0))
    }

    #[allow(dead_code)]
    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((*self * -1.0).dot(n), 1.0);
        let ray_out_perp = (*self + (n * cos_theta)) * etai_over_etat;
        let ray_out_para = n * ((1.0 - ray_out_perp.len_squared()).abs().sqrt() * -1.0);
        ray_out_perp + ray_out_para
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64 {
        self.z
    }

    #[allow(dead_code)]
    pub fn len_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    #[allow(dead_code)]
    pub fn dot(&self, _rhs: Vec3) -> f64 {
        (self.x * _rhs.x) + (self.y * _rhs.y) + (self.z * _rhs.z)
    }

    #[allow(dead_code)]
    pub fn cross(&self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * _rhs.z) - (self.z * _rhs.y),
            y: (self.z * _rhs.x) - (self.x * _rhs.z),
            z: (self.x * _rhs.y) - (self.y * _rhs.x),
        }
    }

    #[allow(dead_code)]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.len()
    }

    #[allow(dead_code)]
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions
        let thresh = 1e-8;
        return (self.x.abs() < thresh) && (self.y.abs() < thresh) && (self.z.abs() < thresh);
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Aliases
pub type Color = Vec3;
pub type Point3 = Vec3;
