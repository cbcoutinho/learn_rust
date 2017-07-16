# A small Rust clone of grep

This is a simple rust clone of `grep`, a file parser. It searches a file, line by line, for the input word.

Use in this directory like this:

```
> cargo run the poem.txt
Compiling greprs v0.1.0 (file:///home/chris/Projects/learn_rust/greprs)
 Finished dev [unoptimized + debuginfo] target(s) in 1.11 secs
  Running `target/debug/greprs the poem.txt`
Then there's a pair of us — don't tell!
To tell your name the livelong day
```

This project follows chapters 12 and 13 of The Rust Programming Language Book: [An I/O Project Building a Small Grep](https://doc.rust-lang.org/book/second-edition/ch12-00-an-io-project.html)
