use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let file = File::create("scene.ppm");
    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    file.write_all(b"P3\n3 2\n255\n255 0 0 0 255 0 0 0 255\n255 255 0 255 255 255 0 0 0")?;

    Ok(())
}
