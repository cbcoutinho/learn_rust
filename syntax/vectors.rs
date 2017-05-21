fn main() {
    // The following two lines are equivalent
    // The first line creates an array explicitly, whereas the
    // second collects all the elements of an iterator and
    // places them into a vector, also infering the type. The
    // third creates a range iterable and unpacks it similar
    // to the second option. The fourth turns the vector into
    // an iterable, maps it into another range and then unpacks
    // it into another vector

    // let v = vec![1, 2, 3, 4, 5];
    // let v = (1..6).collect::<Vec<_>>();
    let rng = 1..6;
    let v = &rng.collect::<Vec<_>>();
    println!("v = {:?}", v);

    let rng = v
            .into_iter()
            .map(|x| (x+1)*2);
    let v = &rng.collect::<Vec<_>>();
    println!("v = {:?}", v);

    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }
}
