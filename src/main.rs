use std::fs::File;
use std::io::prelude::*;

mod image;
mod pixel;
mod vec3;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let my_image = image::Image::create_white(1920, 1080);
    my_image.writeToFile("full_hd.ppm");

    let my_vec = vec3::Vec3 {
        x: 255.0,
        y: 255.0,
        z: 0.0,
    };

    let my_pixel = pixel::Pixel {
        r: 190,
        g: 133,
        b: 254,
    };

    let file = File::create("scene6.ppm");
    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    file.write_all(b"P3\n3 2\n255\n 255 0 0 255 0 0 255 0 0\n")?;

    write!(file, "{} {} {} ", my_pixel.r, my_pixel.g, my_pixel.b).expect("error");
    write!(file, "{} {} {} ", my_pixel.r, my_pixel.g, my_pixel.b).expect("error");
    write!(file, "{} {} {} ", my_pixel.r, my_pixel.g, my_pixel.b).expect("error");

    Ok(())
}
