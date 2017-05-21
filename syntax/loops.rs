fn main() {
    // Basic while loop
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // Basic for loop
    // NOTE: loop using an iterator, not an array
    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    // Get the index value using '.enumerate'
    // index and value are in a tuple
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    // Convert strings to lines (?)
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    // End an infinite loop (also just called `loop`) using break
    // This is instead of a while loop
    let mut x = 5;

    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 { break; }
    }

    // Using `continue` makes it possible to skip remaining portion of loop
    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }

    // Pretty cool usage of naming loops:
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }
}
