extern crate image;
extern crate noisy_float;

use self::noisy_float::prelude::*;

mod interval;
mod sorting;


pub struct Config {
    pub image_input_path: String,
    output_image_path: String,
    pub interval_function: fn(pixels: &Vec<Vec<image::Rgba<u8>>>, args: &Config) -> Vec<Vec<u32>>,
    interval_file_path: Option<String>,
    bottom_threshold: f32,
    upper_threshold: f32,
    clength: u32,
    angle: u32,
    pub randomness: u32,
    pub sorting_function: fn(pixel: &image::Rgba<u8>) -> N32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        Ok(
            Config {
                image_input_path: args[1].clone(),
                output_image_path: String::from("output.png"),
                interval_function: interval::threshold,
                interval_file_path: None,
                bottom_threshold: 0.25,
                upper_threshold: 0.8,
                clength: 50,
                angle: 0,
                randomness: 0,
                sorting_function: sorting::lightness,
            }
        )
    }
}
