use super::ray::{HitRecord, Hittable, Ray};

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, o: Box<dyn Hittable>) {
        self.objects.push(o)
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(hr) => {
                    closest_so_far = hr.t();
                    hit_record = Some(hr);
                }
                None => (),
            }
        }

        hit_record
    }
}
