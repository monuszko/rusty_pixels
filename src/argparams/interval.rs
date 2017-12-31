use image;
use super::Config;
use super::sorting::util;

pub fn threshold(pixels: &Vec<Vec<image::Rgba<u8>>>, args: &Config) -> Vec<Vec<u32>> {
    let mut intervals: Vec<Vec<u32>> = Vec::new();

    println!("Defining intervals...");
    for y in 0..pixels.len() {
        let mut interval: Vec<u32> = Vec::new();
        for x in 0..pixels[0].len() {
            if util::lightness(&pixels[y][x]) < args.lower || util::lightness(&pixels[y][x]) > args.upper {
                interval.push(x as u32);
            }
        }
        interval.push(pixels[0].len() as u32);
        intervals.push(interval);
        }
    intervals
    }



// TODO: seems to enter an endless loop with clength = 1
pub fn random(pixels: &Vec<Vec<image::Rgba<u8>>>, args: &Config) -> Vec<Vec<u32>> {
    let mut intervals: Vec<Vec<u32>> = Vec::new();

    println!("Defining intervals...");

    for _y in 0..pixels.len() {
        let mut interval: Vec<u32> = Vec::new();
        let mut x = 0;
        loop {
            let width = util::random_width(args.clength);
            x += width;
            if x > pixels[0].len() as u32 {
                interval.push(pixels[0].len() as u32);
                break;
            } else {
                interval.push(x as u32);
            }
        }
        intervals.push(interval);
    }
    intervals
}
