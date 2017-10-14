extern crate comp;

use std::env;
use std::process;
use std::io::prelude::*;

use comp::Config;

fn main() {
    // Collect command-line args into a vector of strings
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Parsing error: {}", err).expect("Could not write to stderr");
        process::exit(1)
    });

    println!("Filename In: {}", config.filename_in);
    println!("Filename Out: {}", config.filename_out);

    if let Err(e) = comp::run(config) {
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    }
}
