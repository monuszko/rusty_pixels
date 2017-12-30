extern crate rusty_pixels;

use std::process;

use rusty_pixels::argparams::Config;


fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = rusty_pixels::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
