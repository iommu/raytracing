mod camera;
mod exporter;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::time::Instant;
use std::{io, rc::Rc};

use camera::Camera;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    // World setup
    let material_ground: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = (a + b + 22) as f64 / 44.0;
            let center = Point3::new(a as f64 + 0.9 * 0.5, 0.2, b as f64 + 0.9 * 0.5);

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Option<Rc<dyn Material>> = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    Some(Rc::new(Lambertian::new(albedo)))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_from_range(0.5, 1.0);
                    let fuzz = 0.5;
                    Some(Rc::new(Metal::new(albedo, fuzz)))
                } else {
                    // Glass
                    Some(Rc::new(Dielectric::new(1.5)))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1: Option<Rc<dyn Material>> = Some(Rc::new(Dielectric::new(1.5)));
    let material_2: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    let material_3: Option<Rc<dyn Material>> =
        Some(Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    // Camera setup
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 10;
    camera.max_depth = 10;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    // Render
    let start = Instant::now();
    camera.render(&world);
    let duration = start.elapsed();
    eprintln!("render time: {:?}", duration);

    Ok(())
}
