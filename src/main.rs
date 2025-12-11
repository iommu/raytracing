mod camera;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod utils;

use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    // World setup
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera setup
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    // Render
    camera.render(&world);
}
