use std::rc::Rc;

use crate::vec3::{Color, Point3};

use derive_new::new as New;

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color;
}

#[derive(Debug, Clone, Copy, New)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    #[allow(dead_code)]
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        SolidColor {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Point3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    #[allow(dead_code)]
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: even,
            odd: odd,
        }
    }

    #[allow(dead_code)]
    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Rc::new(SolidColor::new(c1)),
            Rc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color {
        let x_int = (self.inv_scale * point.x).floor() as i32;
        let y_int = (self.inv_scale * point.y).floor() as i32;
        let z_int = (self.inv_scale * point.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        return if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        };
    }
}
