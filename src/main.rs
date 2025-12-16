// #![allow(dead_code, unused_variables, unused_imports)]
mod aabb;
mod bvh;
mod camera;
mod exporter;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtw_image;
mod sphere;
mod texture;
mod utils;
mod vec3;

use std::time::Instant;
use std::{io, rc::Rc};

use bvh::BVHNode;
use camera::Camera;
use exporter::{BMPExporter, Exporter};
use hittable::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use utils::{random_double, random_double_range};
use vec3::{Color, Point3, Vec3};

use crate::texture::{CheckerTexture, ImageTexture, Texture};

fn earth() -> io::Result<()> {
    let exporter: Box<dyn Exporter> = Box::new(BMPExporter::new("render.bmp")?);
    let mut world = HittableList::default();
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg")) as Rc<dyn Texture>;
    let earth_surface = Rc::new(Lambertian::new(earth_texture)) as Rc<dyn Material>;
    let globe = Rc::new(Sphere::new_stationary(
        Point3::default(),
        2.0,
        earth_surface,
    ));
    world.add(globe);

    let mut camera = Camera::from_exporter(exporter);
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 10;
    camera.max_depth = 10;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 12.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    // Render
    camera.render(&world);

    Ok(())
}

fn bouncing() -> io::Result<()> {
    let exporter: Box<dyn Exporter> = Box::new(BMPExporter::new("render.bmp")?);

    let mut world = HittableList::default();

    // World setup
    let material_ground: Rc<dyn Material> =
        Rc::new(Lambertian::from_color(Color::new(0.5, 0.5, 0.5)));
    let material_checker: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(
        CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)),
    )));

    world.add(Rc::new(Sphere::new_stationary(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_checker,
    )));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = (a + b + 22) as f64 / 44.0;
    //         let center = Point3::new(a as f64 + 0.9 * 0.5, 0.2, b as f64 + 0.9 * 0.5);

    //         if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
    //             let sphere = if choose_mat < 0.8 {
    //                 // Diffuse
    //                 let albedo = Color::random() * Color::random();
    //                 let center_2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
    //                 Sphere::new_moving(
    //                     center,
    //                     center_2,
    //                     0.2,
    //                     Rc::new(Lambertian::new(albedo)),
    //                 )
    //             } else if choose_mat < 0.95 {
    //                 // Metal
    //                 let albedo = Color::random_from_range(0.5, 1.0);
    //                 let fuzz = 0.5;
    //                 Sphere::new_stationary(center, 0.2, Rc::new(Metal::new(albedo, fuzz)))
    //             } else {
    //                 // Glass
    //                 Sphere::new_stationary(center, 0.2, Rc::new(Dielectric::new(1.5)))
    //             };

    //             world.add(Rc::new(sphere));
    //         }
    //     }
    // }

    let material_1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let material_2: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    let material_3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Rc::new(Sphere::new_stationary(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    world.add(Rc::new(Sphere::new_stationary(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    world.add(Rc::new(Sphere::new_stationary(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world = HittableList::new(Rc::new(BVHNode::from_list(world)));
    // Camera setup
    let mut camera = Camera::from_exporter(exporter);
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

fn main() -> io::Result<()> {
    earth()
}
