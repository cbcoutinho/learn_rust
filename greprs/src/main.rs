use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    // Collect command-line args into a vector of strings
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough input arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}
