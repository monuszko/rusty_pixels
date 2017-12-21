extern crate image;
extern crate rand;

use std::fs::File;
use std::env;

use image::GenericImage;

mod argparams;
mod sorter;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = argparams::Config::new(&args);

    println!("Opening image...");
    let mut img = image::open("camels-morocco.jpg").unwrap();

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
}
