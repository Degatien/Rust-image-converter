use std::env;
use std::process;

use rust_image_converter::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Looking for file: {}", config.file_path);
    println!("to convert to format: {}", config.format);
    println!("and to save as: {}", config.dest_path);
}

