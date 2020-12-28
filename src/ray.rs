use super::color::Color;
use super::vec3::{Point3, Vec3};
use std::rc::Rc;

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

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new<'a>(
        t: f64,
        p: Point3,
        outward_normal: &Vec3,
        r: &Ray,
        material: Rc<dyn Material>,
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct ScatterRecord {
    scattered: Ray,
    attenuation: Color,
}

impl ScatterRecord {
    pub fn new(scattered: Ray, attenuation: Color) -> Self {
        Self {
            scattered,
            attenuation,
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<ScatterRecord>;
}
