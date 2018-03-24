extern crate clap;
use clap::App;

fn main() {
    App::new("par")
        .version("0.1.0")
        .about("Text (re)formatter")
        .author("Chris Coutinho")
        .get_matches();

    println!("Hello World!")
}
