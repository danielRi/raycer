use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Doe Ray Egon Tracer",
    about = "Ray Tracer written by Daniel Riegler"
)]
pub struct Cli {
    #[structopt(short, long, default_value = "build/image.ppm")]
    pub output: PathBuf,

    #[structopt(short, long, default_value = "192")]
    pub width: u32,

    #[structopt(short, long, default_value = "108")]
    pub height: u32,

    #[structopt(short, long, default_value = "0")]
    pub red: u8,

    #[structopt(short, long, default_value = "0")]
    pub green: u8,

    #[structopt(short, long, default_value = "0")]
    pub blue: u8,
}
