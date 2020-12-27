use rand::Rng;
use std::io::Write;

mod camera;
mod color;
mod ray;
mod scene;
mod sphere;
mod vec3;

use color::Color;
use scene::Scene;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let samples_per_pixel = 20;

    let camera = camera::Camera::new();

    let header = format!(
        "P3
{} {}
255
",
        image_width, image_height
    );
    print!("{}", header);

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.)));

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range((0.)..1.)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range((0.)..1.)) / (image_height - 1) as f64;
                let r = camera.ray_to(u, v);
                pixel_color += r.color(&scene);
            }

            // Divide the color by the number of samples.
            pixel_color *= 1.0 / samples_per_pixel as f64;

            pixel_color.write();
        }
    }

    eprintln!("\rDone.");
}
