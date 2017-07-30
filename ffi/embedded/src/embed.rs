use std::thread;

#[no_mangle]
pub extern "C" fn process() {
    process_fn()
}

pub fn process_fn() {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let mut x = 0;
                for _ in 0..5_000_000 {
                    x += 1
                }
                x
            })
        })
        .collect();

    for h in handles {
        println!(
            "Thread finished with count={}",
            h.join().map_err(|_| "Could not join a thread!").unwrap()
        );
    }
    println!("done!");
}

// #[no_mangle]
// pub extern fn treble(value: i32) -> i32 {
//     value * 3
// }
