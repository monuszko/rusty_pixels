use image;
use noisy_float::prelude::*;
use palette::{Rgb, Hsv, FromColor};
use rand::{thread_rng, Rng};


pub fn lightness(pixel: &image::Rgba<u8>) -> N32 {
    let prgb = Rgb::new_u8(pixel.data[0], pixel.data[1], pixel.data[2]);
    n32(Hsv::from_rgb(prgb).value)
}


pub fn random_width(clength: u32) -> u32 {
    let mut rng = thread_rng();
    let x = rng.next_f32();

    (clength as f32 * (1.0 - x)) as u32
}
