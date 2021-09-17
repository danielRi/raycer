use indicatif::ProgressBar;
use structopt::StructOpt;

use crate::pixel::Pixel;

mod cli;
mod image;
mod pixel;
mod ray;
pub mod vec3;

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::from_args();

    // create empty white image
    let aspect_ratio: f64 = cli.width as f64 / cli.height as f64;
    let create_image_progress_bar = ProgressBar::new((cli.width * cli.height).into());
    let write_file_progress_bar = ProgressBar::new((cli.width * cli.height).into());

    let mut my_image = image::Image::create(
        cli.width,
        cli.height,
        cli.red,
        cli.green,
        cli.blue,
        &|progress| create_image_progress_bar.inc(progress),
    );

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0; // distance between camera and "canvas"
    let camera_origin = vec3::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = vec3::Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = vec3::Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = camera_origin
        .sub(horizontal.div(2.0))
        .sub(vertical.div(2.0))
        .sub(vec3::Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        });

    // render
    for row_index in 0..100 {
        my_image
            .set_pixel(row_index, 100, Pixel { r: 0, g: 20, b: 0 })
            .unwrap();
    }

    // output image to ppm file
    match my_image.write_to_file(
        cli.output.into_os_string().into_string().unwrap(),
        &|progress| write_file_progress_bar.inc(progress),
    ) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut vec3 = vec3::Vec3 {
            x: 0.5,
            y: 0.4,
            z: 0.3,
        };
        let vec4 = vec3::Vec3 {
            x: 0.5,
            y: 0.4,
            z: 0.3,
        };
        vec3 = vec3.add(vec4);

        assert_eq!(vec3.x, 1.0);
        assert_eq!(vec3.y, 0.8);
        assert_eq!(vec3.z, 0.6);

        vec3 = vec3.mul(2.0);

        assert_eq!(vec3.x, 2.0);
        assert_eq!(vec3.y, 1.6);
        assert_eq!(vec3.z, 1.2);

        vec3 = vec3.div(2.0);

        assert_eq!(vec3.x, 1.0);
        assert_eq!(vec3.y, 0.8);
        assert_eq!(vec3.z, 0.6);

        let length_squared = vec3.length_squared();

        assert_eq!(length_squared, 2.0);

        let length = vec3.length();

        assert_eq!(length, 1.4142135623730951);
    }
}
