use super::pixel::Pixel;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    // Factory constructor
    pub fn create(width: u32, height: u32, red: u8, green: u8, blue: u8) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        let mut rows: Vec<Vec<Pixel>> = Vec::new();
        let bar = ProgressBar::new((width * height).into());

        while rows.len() < height as usize {
            let mut row = Vec::new();
            while row.len() < width as usize {
                row.push(Pixel {
                    r: red,
                    g: green,
                    b: blue,
                });
                bar.inc(1);
            }
            rows.push(row);
        }
        Image {
            width: width,
            height: height,
            pixels: rows,
        }
    }

    pub fn write_to_file(&self, path: String) -> Result<()> {
        let file = File::create(path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let bar = ProgressBar::new((self.width * self.height).into());
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                write!(file, "{} {} {} ", pixel.r, pixel.g, pixel.b)?;
                bar.inc(1);
            }
            write!(file, "\n")?;
        }

        Ok(())
    }
}
