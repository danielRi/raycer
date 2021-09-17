use super::pixel::Pixel;

use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.add(self.direction.mul(t))
    }

    pub fn pixel(&self) -> Pixel {
        // TODO: linear interpolation
        Pixel { r: 0, g: 0, b: 0 }
    }
}
