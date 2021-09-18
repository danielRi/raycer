use super::pixel::Pixel;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result as IOResult;
use std::result::Result;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    // Factory constructor
    pub fn create(
        width: u32,
        height: u32,
        red: u8,
        green: u8,
        blue: u8,
        update_progress: &dyn Fn(u64),
    ) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        let mut rows: Vec<Vec<Pixel>> = Vec::new();

        while rows.len() < height as usize {
            let mut row = Vec::new();
            while row.len() < width as usize {
                row.push(Pixel {
                    r: red,
                    g: green,
                    b: blue,
                });
                update_progress(1);
            }
            rows.push(row);
        }
        Image {
            width: width,
            height: height,
            pixels: rows,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) -> Result<(), String> {
        // println!(
        //     "checking {} to {}, real length: {}",
        //     x,
        //     self.width,
        //     self.pixels[y as usize].len()
        // );
        if x >= self.width {
            let error = format!("Invalid x, x is {}, but width is {}", x, self.width);
            return Err(error);
        }
        // println!(
        //     "checking {} to {}, real length is {}",
        //     y,
        //     self.height,
        //     self.pixels.len()
        // );
        if y >= self.height {
            let error = format!("Invalid y, y is {}, but height is {}", y, self.height);
            return Err(error);
        }

        // TODO: set pixel of pixel arrays
        self.pixels[y as usize][x as usize] = pixel;
        Ok(())
    }

    pub fn write_to_file(&self, path: String, update_progress: &dyn Fn(u64)) -> IOResult<()> {
        let file = File::create(path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                write!(file, "{} {} {} ", pixel.r, pixel.g, pixel.b)?;
                update_progress(1);
            }
            write!(file, "\n")?;
        }

        Ok(())
    }
}
