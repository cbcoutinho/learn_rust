extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    // Collect command-line args into a vector of strings
    // let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Parsing error: {}", err).expect("Could not write to stderr");
        process::exit(1)
    });


    if let Err(e) = greprs::run(config) {
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    }

}
