use noisy_float::prelude::*;
use clap;
use image::Rgba;

mod interval;
mod sorting;


fn get_args() -> clap::ArgMatches<'static> {
    clap::App::new("rusty_pixels")
        .version("0.3.0")
        .author("Marek Onuszko <marek.onuszko@gmail.com>")
        .about("mangles images to create glitch art (pixel sorting)")
        .arg(clap::Arg::with_name("INPUT")
            .help("Sets the image file to use")
            .required(true)
            .index(1)
            )
        .arg(clap::Arg::with_name("OUTPUT")
            .short("o")
            .help("Output filename")
            .default_value("test.png")
            )
        .arg(clap::Arg::with_name("lower")
            .help("How dark must a pixel be to be considered as a 'border'
for sorting ?  Takes values from 0-1.
Used in edges and threshold modes.")
            .short("t")
            .default_value("0.25")
            .long("lower")
            )
        .arg(clap::Arg::with_name("upper")
            .help("How bright must a pixel be to be considered as a 'border'
for sorting ?  Takes values from 0-1.
Used in edges and threshold modes.")
            .short("u")
            .default_value("0.8")
            .long("upper")
            )
        .arg(clap::Arg::with_name("randomness")
            .help("What percentage of intervals *not* to sort.")
            .short("r")
            .default_value("0")
            .long("randomness")
            )
        .arg(clap::Arg::with_name("clength")
            .help("Characteristic length for the random width generator.
Used in modes: random, waves")
            .short("c")
            .long("character-length")
            .default_value("50")
            )
        .arg(clap::Arg::with_name("interval")
            .short("i")
            .long("interval-function")
            .possible_values(&["threshold", "random"])
            .default_value("threshold")
            )
    .get_matches()
}


pub struct Config {
    pub input: String,
    pub output: String,
    pub interval_function: fn(pixels: &Vec<Vec<Rgba<u8>>>, args: &Config) -> Vec<Vec<u32>>,
    interval_file_path: Option<String>,
    lower: f32,
    upper: f32,
    clength: u32,
    angle: u32,
    pub randomness: u32,
    pub sorting_function: fn(pixel: &Rgba<u8>) -> N32,
}

impl Config {
    pub fn new() -> Result<Config, clap::Error> {
        let matches = get_args();

        let input = String::from(matches.value_of("INPUT").unwrap());
        let output = String::from(matches.value_of("OUTPUT").unwrap());

        let lower = value_t!(matches, "lower", f32)?;
        let upper = value_t!(matches, "upper", f32)?;
        let randomness = value_t!(matches, "randomness", u32)?;
        let clength = value_t!(matches, "clength", u32)?;
        let interval_function = match matches.value_of("interval").unwrap() {
            "random" => interval::random,
            _        => interval::threshold,
        };

        Ok(
            Config {
                input,
                output,
                interval_function,
                interval_file_path: None,
                lower,
                upper,
                clength,
                angle: 0,
                randomness,
                sorting_function: sorting::lightness,
            }
        )
    }
}
