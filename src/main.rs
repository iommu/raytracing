mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use hittable::HittableList;
use material::{Lambertian, Material, Metal};
use sphere::Sphere;
use vec3::{Color, Vec3};

fn main() {
    // World setup
    let material_ground: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))));
    let material_center: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))));
    let material_left: Option<Rc<dyn Material>> =
        Some(Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3)));
    let material_right: Option<Rc<dyn Material>> =
        Some(Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        &Vec3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(Box::new(Sphere::new(
        &Vec3::new(0.0, 0.0, -1.2),
        0.5,
        &material_center,
    )));
    world.add(Box::new(Sphere::new(
        &Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));
    world.add(Box::new(Sphere::new(
        &Vec3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    // Camera setup
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    // Render
    camera.render(&world);
}
