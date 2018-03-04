use std::io::stdin;

pub fn test() {
    println!("Hello world!");

    let x = ::rand::random::<u8>();
    println!("{}", x);
}

pub fn read_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name);
    name
}
