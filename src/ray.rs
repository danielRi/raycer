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

    pub fn color_for_ray(&self) -> Pixel {
        // TODO: linear interpolation

        let interpolated_color = 255.0 * ((self.direction.y + 1.0) / 2.0);
        println!("y: {}, color: {}", self.direction.y, interpolated_color);
        let interpolated_color = interpolated_color as u8;
        println!("u8: {}", interpolated_color);
        Pixel {
            r: interpolated_color,
            g: interpolated_color,
            b: 255,
        }
    }
}
