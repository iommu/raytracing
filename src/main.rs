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

use crate::vec3::Point3;

fn main() {
    let mut world = HittableList::new();

    let R = (PI / 4.0).cos();

    // World setup
    let material_ground: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))));
    let material_center: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))));
    let material_left: Option<Rc<dyn Material>> = Some(Rc::new(Dielectric::new(1.50)));
    let material_bubble: Option<Rc<dyn Material>> = Some(Rc::new(Dielectric::new(1.0 / 1.50)));
    let material_right: Option<Rc<dyn Material>> =
        Some(Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)));

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
        &Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        &material_bubble,
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
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;

    // Render
    camera.render(&world);
}
