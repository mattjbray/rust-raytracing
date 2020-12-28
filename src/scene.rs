use super::ray::{Hit, Hittable, Ray};

pub struct Scene<'a> {
    objects: Vec<&'a (dyn Hittable)>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, o: &'a (dyn Hittable)) {
        self.objects.push(o)
    }
}

impl<'a> Hittable for Scene<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
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
