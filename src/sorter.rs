use rand::{thread_rng, Rng};
use noisy_float::prelude::*;
use image;

use argparams::Config;


pub fn sort_image(pixels: Vec<Vec<image::Rgba<u8>>>, intervals: Vec<Vec<u32>>, args: &Config) -> Vec<Vec<image::Rgba<u8>>> {
    let mut rng = thread_rng();
    let mut sorted_pixels: Vec<Vec<image::Rgba<u8>>> = Vec::new();

    for y in 0..pixels.len() {
        let mut row: Vec<image::Rgba<u8>> = Vec::new();
        let mut x_min = 0;
        for x_max in intervals[y].iter() {
            //TODO: variable name 'interval' confusing - this one holds pixels.
            let mut interval: Vec<image::Rgba<u8>> = Vec::new();
            for x in x_min..*x_max {
                interval.push(pixels[y as usize][x as usize]);
            }
            if rng.gen_range(0, 101) >= args.randomness {
                row.extend(sort_interval(interval, args.sorting_function));
            }
            else {
                row.extend(interval);
            }
            x_min = *x_max;            
        }
        row.push(pixels[y][0]);
        sorted_pixels.push(row);
    }
    sorted_pixels
}


// TODO: rename function to sorted_interval ?
fn sort_interval<F>(mut interval: Vec<image::Rgba<u8>>, sorting_function: F) -> Vec<image::Rgba<u8>> 
    where F: Fn(&image::Rgba<u8>) -> N32 {
    if interval.is_empty() {
        return interval
    }
    interval.sort_by_key(sorting_function);
    
    interval
}

