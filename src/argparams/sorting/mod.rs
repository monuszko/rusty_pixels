use image;
use noisy_float;

pub mod util;


pub fn lightness(pixel: &image::Rgba<u8>) -> noisy_float::types::N32 {
    util::lightness(pixel)
}
