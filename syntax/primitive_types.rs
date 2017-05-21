fn main() {
    // Booleans
    let x = true;
    let y: bool = false;

    if x {
        println!("x is {}", x);
    }

    if !y {
        println!("y is {}", y);
    }

    // Strings are by default 4 bytes (UTF-compatible)
    let x = 'x';
    let two_hearts = '💕';
    println!("x and two_hearts: {}, {}", x, two_hearts);

    // Arrays are essentially lists (only 1 type allowed, a la python list)
    let a = [1, 2, 3];
    let b = [0; 10];
    println!("a has {} elements", a.len() );
    println!("b has {} elements", b.len() );

    // Listing an entire array in a print statement requires {:?} for some reason (debug mode?)
    println!("The second element of {:?} is {}", a, a[1] );

    // Access element in list using subscript brackets
    // NOTE: arrays are zero-indexed
    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    println!("The second name is: {}", names[1]);

    // Tuples: basically like python tuples, but you can declare each type of the element
    let x = (1, "hello"); // x: (i32, &str)
    let x: (i32, &str) = (1, "hello");

    // Accessing a tuple is different than an array
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("'tuple' is {:?}", tuple);
    println!("x is {}", x);


}
