extern crate image;
extern crate noisy_float;

use self::noisy_float::prelude::*;

pub mod util;


pub fn lightness(pixel: &image::Rgba<u8>) -> noisy_float::types::N32 {
    util::lightness(pixel)
}
