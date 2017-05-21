/// This is a sample comment function
fn main() {

    // This is just a comment
    let x = 1;
    println!("{} + 1 = {}", x, add_one(x));

}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
