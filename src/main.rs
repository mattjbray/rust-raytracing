use rand::Rng;
use std::io::Write;
use std::rc::Rc;

mod camera;
mod color;
mod materials;
mod ray;
mod scene;
mod sphere;
mod vec3;

use color::Color;
use materials::{Lambertian, Metal};
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

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    scene.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    scene.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    scene.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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
