use super::color::Color;
use super::ray::{HitRecord, Material, Ray, ScatterRecord};
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
        hit: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<ScatterRecord> {
        let mut scatter_direction = hit.normal() + Vec3::gen_in_unit_sphere(rng);
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal();
        }
        let scattered = Ray::new(hit.position(), scatter_direction);
        let attenuation = self.albedo;

        Some(ScatterRecord::new(scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit: &HitRecord,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> Option<ScatterRecord> {
        // vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let reflected = r_in.direction().unit().reflect(&hit.normal());
        let scattered = Ray::new(hit.position(), reflected);
        let attenuation = self.albedo;
        if scattered.direction().dot(&hit.normal()) > 0. {
            Some(ScatterRecord::new(scattered, attenuation))
        } else {
            None
        }
    }
}
