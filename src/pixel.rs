pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
