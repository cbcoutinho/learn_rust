use std::error::Error;
use std::fs::File;
// use std::io::prelude::*;
use std::io::Read;

pub struct Config {
    pub filename_in: String,
    pub filename_out: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename_in = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename_in string"),
        };

        let filename_out = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename_out string"),
        };

        Ok(Config {
            filename_in: filename_in,
            filename_out: filename_out,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename_in)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}
