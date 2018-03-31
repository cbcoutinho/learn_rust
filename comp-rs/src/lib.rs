use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

pub struct Config {
    pub filename_in: String,
    pub filename_out: String,
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

    // Put unique occurrences into a BTreeMap
    let mut btmap = BTreeMap::new();

    let num_chars = contents.chars().count();

    println!("Total number of chars: {}", num_chars);
    for c in contents.chars() {
        *btmap.entry(c).or_insert(0) += 1;
    }

    println!("Number of occurrences of each character");
    for (key, value) in &btmap {
        println!(
            "{:?}: {:?} ({:.2}%)",
            key,
            value,
            100.0 * (*value as f64) / (num_chars as f64)
        );
    }

    // Put contents into a Vec to order by value
    let mut v = Vec::from_iter(btmap);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    // Print key-value pair of input file
    println!("\nNumber of occurrences of each character, sorted by value");
    for &(key, value) in v.iter() {
        println!(
            "{:?}: {:?} ({:.2}%)",
            key,
            value,
            100.0 * (value as f64) / (num_chars as f64)
        );
    }

    Ok(())
}
