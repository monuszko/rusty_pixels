use image;
use noisy_float::prelude::*;
use palette::{Rgb, Hsv, FromColor};
use rand::{thread_rng, Rng};


pub fn hsv_hue(pixel: &image::Rgba<u8>) -> N32 {
    let prgb = Rgb::new_u8(pixel.data[0], pixel.data[1], pixel.data[2]);
    n32(Hsv::from_rgb(prgb).hue.to_degrees())
}

pub fn hsv_saturation(pixel: &image::Rgba<u8>) -> N32 {
    let prgb = Rgb::new_u8(pixel.data[0], pixel.data[1], pixel.data[2]);
    n32(Hsv::from_rgb(prgb).saturation)
}

pub fn hsv_value(pixel: &image::Rgba<u8>) -> N32 {
    let prgb = Rgb::new_u8(pixel.data[0], pixel.data[1], pixel.data[2]);
    n32(Hsv::from_rgb(prgb).value)
}

pub fn rgb_minimum(pixel: &image::Rgba<u8>) -> N32 {
    n32(*pixel.data.iter().min().unwrap() as f32)
}


pub fn random_width(clength: u32) -> u32 {
    let mut rng = thread_rng();
    let x = rng.next_f32();

    (clength as f32 * (1.0 - x)) as u32
}
