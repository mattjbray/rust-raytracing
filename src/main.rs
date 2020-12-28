use rand::Rng;
use std::io::Write;

mod camera;
mod color;
mod materials;
mod ray;
mod scene;
mod sphere;
mod vec3;

use color::Color;
use materials::{Dielectric, Lambertian, Metal};
use scene::Scene;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = camera::Camera::new();

    let header = format!(
        "P3
{} {}
255
",
        image_width, image_height
    );
    print!("{}", header);

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let mut scene = Scene::new();
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    scene.add(&ground);

    let center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    scene.add(&center);

    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    scene.add(&left);

    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, &material_right);
    scene.add(&right);

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range((0.)..1.)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range((0.)..1.)) / (image_height - 1) as f64;
                let r = camera.ray_to(u, v);
                pixel_color += r.color(&scene, &mut rng, max_depth);
            }

            // Divide the color by the number of samples.
            pixel_color *= 1.0 / samples_per_pixel as f64;

            pixel_color.write();
        }
    }

    eprintln!("\rDone.");
}
