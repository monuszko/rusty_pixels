use image;
use noisy_float;

pub mod util;


pub fn hue(pixel: &image::Rgba<u8>) -> noisy_float::types::N32 {
    util::hsv_hue(pixel)
}

pub fn saturation(pixel: &image::Rgba<u8>) -> noisy_float::types::N32 {
    util::hsv_saturation(pixel)
}

pub fn lightness(pixel: &image::Rgba<u8>) -> noisy_float::types::N32 {
    util::hsv_value(pixel)
}

pub fn minimum(pixel: &image::Rgba<u8>) -> noisy_float::types::N32 {
    util::rgb_minimum(pixel)
}
