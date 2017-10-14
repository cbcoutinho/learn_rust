extern crate comp_rs;

use std::env;
use std::process;
use std::io::prelude::*;

fn main() {
    // Collect command-line args into a vector of strings
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Parsing error: {}", err).expect("Could not write to stderr");
        process::exit(1)
    });
}
