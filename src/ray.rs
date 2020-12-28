use super::color::Color;
use super::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }

    pub fn color(
        &self,
        scene: &impl Hittable,
        rng: &mut rand::rngs::ThreadRng,
        depth: u32,
    ) -> Color {
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }
        match scene.hit(self, 0.001, std::f64::INFINITY) {
            Some(hit) => match hit.material.scatter(self, &hit, rng) {
                Some(scatter) => {
                    scatter.attenuation * scatter.scattered.color(scene, rng, depth - 1)
                }
                None => Color::new(0., 0., 0.),
            },
            None => {
                // background
                let t = 0.5 * (self.direction.unit().y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

pub struct Hit<'a> {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: &'a (dyn Material),
}

impl<'a> Hit<'a> {
    pub fn new(
        t: f64,
        p: Point3,
        outward_normal: &Vec3,
        r: &Ray,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn position(&self) -> Point3 {
        self.p
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Scatter {
    scattered: Ray,
    attenuation: Color,
}

impl Scatter {
    pub fn new(scattered: Ray, attenuation: Color) -> Self {
        Self {
            scattered,
            attenuation,
        }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit, rng: &mut rand::rngs::ThreadRng) -> Option<Scatter>;
}
