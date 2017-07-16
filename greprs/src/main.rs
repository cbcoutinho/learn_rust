use std::env;

fn main() {
    // Collect command-line args into a vector of strings
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);


}
