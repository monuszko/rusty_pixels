use noisy_float::prelude::*;
use clap::{App, Arg};
use image::Rgba;

mod interval;
mod sorting;


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
    pub fn new() -> Result<Config, &'static str> {
    let matches = App::new("rusty_pixels")
            .version("0.2.0")
            .author("Marek Onuszko <marek.onuszko@gmail.com>")
            .about("mangles images to create glitch art (pixel sorting)")
            .arg(Arg::with_name("INPUT")
                .help("Sets the image file to use")
                .required(true)
                .index(1)
                )
            .arg(Arg::with_name("OUTPUT")
                .short("o")
                .help("Output filename")
                .default_value("test.png")
                )
            .arg(Arg::with_name("lower")
                .help("How dark must a pixel be to be considered as a 'border'
for sorting ?  Takes values from 0-1, 0.25 by default.
Used in edges and threshold modes.")
                .short("t")
                .default_value("0.25")
                .long("lower")
                )
            .arg(Arg::with_name("upper")
                .help("How bright must a pixel be to be considered as a 'border'
for sorting ?  Takes values from 0-1, 0.8 by default.
Used in edges and threshold modes."
                )
            .short("u")
            .default_value("0.8")
            .long("upper")
                )
            .arg(Arg::with_name("randomness")
                .help("What percentage of intervals *not* to sort. 0 by default.")
            .short("r")
            .default_value("0")
            .long("randomness")
                )
        .get_matches();

        let input = String::from(matches.value_of("INPUT").unwrap());
        let output = String::from(matches.value_of("OUTPUT").unwrap());
        let lower = value_t!(matches, "lower", f32).unwrap();
        let upper = value_t!(matches, "upper", f32).unwrap();
        let randomness = value_t!(matches, "randomness", u32).unwrap();
        Ok(
            Config {
                input: input,
                output: output,
                interval_function: interval::threshold,
                interval_file_path: None,
                lower: lower,
                upper: upper,
                clength: 50,
                angle: 0,
                randomness: randomness,
                sorting_function: sorting::lightness,
            }
        )
    }
}
