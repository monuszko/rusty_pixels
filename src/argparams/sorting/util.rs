extern crate image;
extern crate noisy_float;
extern crate palette;

use self::noisy_float::prelude::*;
use self::palette::{FromColor, Hsv, Rgb};


pub fn lightness(pixel: &image::Rgba<u8>) -> N32 {
    let prgb = palette::Rgb::new_u8(pixel.data[0], pixel.data[1], pixel.data[2]);
    n32(Hsv::from_rgb(prgb).value)
}
