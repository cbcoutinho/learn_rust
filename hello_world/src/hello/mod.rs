use std::io::stdin;

mod sub_hello;

pub fn test() {
    sub_hello::test2();
}

pub fn read_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name);
    name
}
