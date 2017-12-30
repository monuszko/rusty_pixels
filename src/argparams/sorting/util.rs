use image;
use noisy_float::prelude::*;
use palette::{Rgb, Hsv, FromColor};


pub fn lightness(pixel: &image::Rgba<u8>) -> N32 {
    let prgb = Rgb::new_u8(pixel.data[0], pixel.data[1], pixel.data[2]);
    n32(Hsv::from_rgb(prgb).value)
}
