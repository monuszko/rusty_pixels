extern crate rusty_pixels;

use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = rusty_pixels::argparams::Config::new(&args).unwrap();
    
    if let Err(e) = rusty_pixels::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
