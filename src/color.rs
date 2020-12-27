pub type Color = super::vec3::Vec3;

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

impl Color {
    pub fn write(&self) {
        let normalize = |v| (256.0 * clamp(v, 0.0, 0.999)) as u8;

        println!(
            "{} {} {}",
            normalize(self.x),
            normalize(self.y),
            normalize(self.z)
        )
    }
}
