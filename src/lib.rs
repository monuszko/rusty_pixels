extern crate image;
extern crate rand;

use std::error::Error;
use std::fs::File;

use image::GenericImage;

pub mod argparams;
mod sorter;


pub fn run(config: argparams::Config) -> Result<(), Box<Error>> {
    println!("Opening image...");
    let mut img = image::open(&config.image_input_path)?;

    //println!("Converting to RGBA...");
    //let img = img.to_rgba();

    // println!("Rotating image...");

    // println!("Getting data...");


    println!("Getting pixels...");
    let mut pixels: Vec<Vec<image::Rgba<u8>>> = Vec::new();
    let width = img.width();
    for y in 0..img.height() {
        let row = img.crop(0, y, width, 1);
        let mut v: Vec<image::Rgba<u8>> = Vec::new();
        for x in 0..width {
            v.push(img.get_pixel(x, y));
        }
        pixels.push(v);
    }

    println!("Determining intervals...");
    let intervals = (config.interval_function)(&pixels, &config);

    println!("Sorting pixels...");
    let sorted_pixels = sorter::sort_image(pixels, intervals, &config);

    println!("Placing pixels in output image...");

    // if argparams.angle is not 0:

    println!("Saving image...");
    let ref mut fout = File::create("test.png").unwrap();


    for x in 0..img.width() {
        for y in 0..img.height() {
            img.put_pixel(x, y, sorted_pixels[y as usize][x as usize]);
        }
    }
    
    img.save(fout, image::PNG).unwrap();

    // output_image_path
    println!("Done!");
Ok(())
}
