use structopt::StructOpt;

mod cli;
mod image;
mod pixel;
mod vec3;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let cli = cli::Cli::from_args();

    let my_image = image::Image::create(cli.width, cli.height, cli.red, cli.green, cli.blue);
    match my_image.write_to_file(cli.output.into_os_string().into_string().unwrap()) {
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
