mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::{f64::consts::PI, rc::Rc};

use camera::Camera;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use vec3::{Color, Vec3};

fn main() {
        let mut world = HittableList::new();

        let R = (PI/4.0).cos();

    // World setup
    let material_left: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))));
    let material_right: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0))));

    world.add(Box::new(Sphere::new(
        &Vec3::new(-R, 0.0, -1.0),
        R,
        &material_left,
    )));
    world.add(Box::new(Sphere::new(
        &Vec3::new(R, 0.0, -1.0),
        R,
        &material_right,
    )));

    // Camera setup
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    
    camera.vfov = 90.0;

    // Render
    camera.render(&world);
}
