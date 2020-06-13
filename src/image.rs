use super::pixel::Pixel;
use std::fs::File;
use std::io::prelude::*;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    // Factory constructor
    pub fn create_white(width: u32, height: u32) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        let amount_of_pixel = width * height;
        let mut pixels: Vec<Pixel> = Vec::new();

        let mut index = 0;
        while index < amount_of_pixel {
            pixels.push(Pixel {
                r: 255,
                g: 255,
                b: 255,
            });
            index = index + 1;
        }
        Image {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn writeToFile(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;
        let mut index_height: usize = 0;
        while index_height < self.height as usize {
            let mut index_width: usize = 0;
            while index_width < self.width as usize {
                let index = index_width + (index_height * self.width as usize);
                /* println!(
                    "index_height: {}, index_width: {}, index {}/{}",
                    index_height,
                    index_width,
                    index,
                    self.pixels.len()
                ); */
                let pixel: &Pixel = &self.pixels[index as usize];
                write!(file, "{} {} {} ", pixel.r, pixel.g, pixel.b)?;
                index_width = index_width + 1;
            }
            index_height = index_height + 1;
        }

        Ok(())
    }
}
