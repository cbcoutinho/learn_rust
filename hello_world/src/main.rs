extern crate rand;

mod hello;

fn main() {
    let name = hello::read_name();
    //let name2 = name;

    println!("name: {}", name); // <- Move error, name2
                                //println!("name2: {}", name2);
}
