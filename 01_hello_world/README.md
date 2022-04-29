## Hello World

### Summary
1. [Basic Example](#basic-example)

### Basic example
So, as usual, let's start with a simple program that writes: _Hello, World!_

```rust
// hello_world.rs

// fn is the keyword that defines functions
// we'll see more about it later on.
// The `main` is the function name
fn main() {
    // println is a macro, that
    // prints something, and then breaks the line.
    println!("Hello, World!");
}
```

To run it is pretty simple:
1. Compiles the program into a binary:
  - > rustc hello_world.rs
2. Then you can run it:
  - > ./hello_world

Congrats! You've written your first Rust program.
