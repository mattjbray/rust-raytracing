use super::ray::{Hit, Hittable, Material, Ray};
use super::vec3::Point3;

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a (dyn Material),
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a (dyn Material)) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let hit_record = Hit::new(t, p, &outward_normal, ray, self.material.clone());
        return Some(hit_record);
    }
}
