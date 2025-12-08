mod ray;
mod vec3;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

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

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().len_squared();
    let h = Vec3::dot(ray.direction(), &oc);
    let c = oc.len_squared() - radius*radius;
    let discriminant = h*h - a*c;
    //
    return if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    };
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let N = Vec3::unit_vector(&(&ray.at(t) - &Vec3::new(0.0, 0.0, -1.0)));
        return &Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }

    let unit_dir = ray.direction().unit_vector();
    let a = 0.5 * (unit_dir.y() + 1.0);

    &(&Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + &(&Color::new(0.5, 0.7, 1.0) * a)
}

fn main() {
    // Config
    let aspect_ration = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height = (image_width as f64 / aspect_ration) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera config
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = &(&(&camera_center - &Vec3::new(0.0, 0.0, focal_length))
        - &(&viewport_u / 2.0))
        - &(&viewport_v / 2.0);
    let pixel00_loc = &viewport_upper_left + &(&(&pixel_delta_u + &pixel_delta_v) * 0.5);

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                &(&pixel00_loc + &(&pixel_delta_u * i as f64)) + &(&pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r);

            write_color(&pixel_color);
        }
    }
}
