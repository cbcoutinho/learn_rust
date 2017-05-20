fn main() {
    let x = true;
    let y: bool = false;

    if x {
        println!("x is {}", x);
    }

    if !y {
        println!("y is {}", y);
    }

    let x = 'x';
    let two_hearts = '💕';

    println!("x and two_hearts: {}, {}", x, two_hearts);
}
