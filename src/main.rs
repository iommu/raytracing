mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;

type Color = Vec3;

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

fn main() {
    // Config
    let image_width = 256;
    let image_height = 256;

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0f64,
            );
            write_color(&pixel_color);
        }
    }
}
