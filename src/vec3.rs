pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        &self.x * &self.x + &self.y * &self.y + &self.z * &self.z
    }

    pub fn add(&self, vec: Vec3) -> Vec3 {
        let x = self.x + vec.x;
        let y = self.y + vec.y;

        let z = self.z + vec.z;
        return Vec3 { x: x, y: y, z: z };
    }

    pub fn sub(&self, vec: Vec3) -> Vec3 {
        let x = self.x - vec.x;
        let y = self.y - vec.y;

        let z = self.z - vec.z;
        return Vec3 { x: x, y: y, z: z };
    }

    pub fn mul(&self, t: f64) -> Vec3 {
        let x = self.x * t;
        let y = self.y * t;
        let z = self.z * t;
        return Vec3 { x: x, y: y, z: z };
    }

    pub fn div(&self, t: f64) -> Vec3 {
        let vec3 = self.mul(1.0 / t);
        return vec3;
    }
}
