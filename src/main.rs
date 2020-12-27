use std::io::Write;

mod camera;
mod color;
mod ray;
mod scene;
mod sphere;
mod vec3;

use scene::Scene;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

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

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = camera.ray_to(u, v);
            let pixel_color = r.color(&scene);

            pixel_color.print();
        }
    }

    eprintln!("\rDone.");
}
