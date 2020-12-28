use super::color::Color;
use super::ray::{Hit, Material, Ray, Scatter};
use super::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        hit: &Hit,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<Scatter> {
        let mut scatter_direction = hit.normal() + Vec3::gen_in_unit_sphere(rng);
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal();
        }
        let scattered = Ray::new(hit.position(), scatter_direction);
        let attenuation = self.albedo;

        Some(Scatter::new(scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit: &Hit,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<Scatter> {
        let reflected = r_in.direction().unit().reflect(&hit.normal());
        let scattered = Ray::new(
            hit.position(),
            reflected + self.fuzz * Vec3::gen_in_unit_sphere(rng),
        );
        let attenuation = self.albedo;
        if scattered.direction().dot(&hit.normal()) > 0. {
            Some(Scatter::new(scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit: &Hit,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = r_in.direction().unit();
        let refracted = unit_direction.refract(&hit.normal(), refraction_ratio);

        let scattered = Ray::new(hit.position(), refracted);

        Some(Scatter::new(scattered, attenuation))
    }
}
