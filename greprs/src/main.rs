use std::env;

fn main() {
    // Collect command-line args into a vector of strings
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // println!("Hello, world!");
}
