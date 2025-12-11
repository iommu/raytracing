use std::f64::INFINITY;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Translate the [0,1] component values to the byte range [0,255]
    let ir = (255.999 * r) as i64;
    let ig = (255.999 * g) as i64;
    let ib = (255.999 * b) as i64;

    // Write out the pixel color components
    print!("{ir} {ig} {ib}\n");
}


pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    pub fn render<T: Hittable>(&mut self, world: &T) {
        self.initialize();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

    for j in 0..self.image_height {
        for i in 0..self.image_width {
            let pixel_center =
                &(&self.pixel00_loc + &(&self.pixel_delta_u * i as f64)) + &(&self.pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &self.center;
            let ray = Ray::new(&self.center, &ray_direction);

            let pixel_color = Self::ray_color(&ray, world);

            write_color(&pixel_color);
        }
    }
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point3::default();

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = &(&(&self.center - &Vec3::new(0.0, 0.0, focal_length))
            - &(&viewport_u / 2.0))
            - &(&viewport_v / 2.0);
        self.pixel00_loc =
            &viewport_upper_left + &(&(&self.pixel_delta_u + &self.pixel_delta_v) * 0.5);
    }

    fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
        let mut rec = HitRecord::new();

        if world.hit(ray, Interval::new(0.0, INFINITY), &mut rec) {
            return &(&rec.normal + &Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_dir = ray.direction().unit_vector();
        let a = 0.5 * (unit_dir.y() + 1.0);

        &(&Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + &(&Color::new(0.5, 0.7, 1.0) * a)
    }
}
