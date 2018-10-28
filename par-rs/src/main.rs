#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let _matches = App::new("par")
        .version(crate_version!())
        //.about("Text (re)formatter")
        .about(crate_description!())
        .author(crate_authors!())
        .get_matches();

    println!("Hello World!")
}
