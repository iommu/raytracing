use std::f64::INFINITY;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Color, Point3, Vec3};

fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = Interval::new(0.0, 0.999);
    let ir = (255.999 * intensity.clamp(r)) as i64;
    let ig = (255.999 * intensity.clamp(g)) as i64;
    let ib = (255.999 * intensity.clamp(b)) as i64;

    // Write out the pixel color components
    print!("{ir} {ig} {ib}\n");
}

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pixel_samples_scale: f64,
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
            samples_per_pixel: 10,
            max_depth: 10,
            pixel_samples_scale: 0.0,
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
                let mut pixel_color = Color::default();
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }
                write_color(&(&pixel_color * self.pixel_samples_scale));
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

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

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

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = &self.pixel00_loc
            + &(&(&self.pixel_delta_u * (i as f64 + offset.x()) as f64)
                + &(&self.pixel_delta_v * (j as f64 + offset.y()) as f64));

        let ray_origin = &self.center;
        let ray_direction = &pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn ray_color<T: Hittable>(ray: &Ray, depth: i32, world: &T) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if world.hit(ray, Interval::new(0.001, INFINITY), &mut rec) {
            let direction = rec.normal.random_on_hemisphere();
            return &Self::ray_color(&Ray::new(&rec.p, &direction), depth - 1, world) * 0.5;
        }

        let unit_dir = ray.direction().unit_vector();
        let a = 0.5 * (unit_dir.y() + 1.0);

        &(&Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + &(&Color::new(0.5, 0.7, 1.0) * a)
    }
}
