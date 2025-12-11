use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn len_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, _rhs: &Vec3) -> f64 {
        (self.x * _rhs.x) + (self.y * _rhs.y) + (self.z * _rhs.z)
    }

    pub fn cross(&self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * _rhs.z) - (self.z * _rhs.y),
            y: (self.z * _rhs.x) - (self.x * _rhs.z),
            z: (self.x * _rhs.y) - (self.y * _rhs.x),
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.len()
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

impl<'a, 'b> ops::Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl<'a> ops::Add<f64> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<'a> ops::Div<f64> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl<'a> ops::Mul<f64> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

// Aliases
pub type Color = Vec3;
pub type Point3 = Vec3;
