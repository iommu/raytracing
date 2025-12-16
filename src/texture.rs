use std::{path::Path, rc::Rc};

use crate::{
    interval::Interval,
    rtw_image::RTWImage,
    vec3::{Color, Point3},
};

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

pub struct ImageTexture {
    image: RTWImage,
}

impl ImageTexture {
    #[allow(dead_code)]
    pub fn new(path: &str) -> Self {
        Self {
            image: RTWImage::new(path).unwrap(),
        }
    }

    
}

impl Texture for ImageTexture {
#[allow(dead_code)]
    fn value(&self, mut u: f64, mut v: f64, point: Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        u = Interval::new(0.0, 1.0).clamp(u);
        v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel = self.image.pixel_at(i, j);

        let color_scale = 1.0 / 255.0;
        return Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        );
    }
}