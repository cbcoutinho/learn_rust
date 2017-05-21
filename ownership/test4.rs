fn main() {

    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with `v1` and `v2`.

        // Return the answer.
        42
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo(&v1, &v2);

    // We can use `v1` and `v2` here!

    println!("v1 = {:?}, v2 = {:?}, answer = {}", v1, v2, answer);
}
