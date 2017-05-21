fn main() {
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    }

    let y = if x == 5 {
        10
    } else {
        15
    };

    println!("y = {}", y);
}
