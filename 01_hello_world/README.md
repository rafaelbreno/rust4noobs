## Hello World

### Summary
1. [Basic Example](#basic-example)
2. [Comments](#comments)
3. [Formatting](#formatting)

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

### Comments
In Rust there's two types of comments:
- One line comments(`// comment`):
```rust
// This is a One Line comment
// This is another One Line comment
```

- Multi-line comments(`/* comment */`):
```rust
/* 
  This is a One Line comment
  This is another One Line comment
*/
```

### Formatting
Printing is handled by a series of [macros](https://doc.rust-lang.org/rust-by-example/macros.html) defined in `std::fmt` some of which include:

- `format!`: write formatted text to String
- `print!`: same as format! but the text is printed to the console (io::stdout).
- `println!`: same as print! but a newline is appended.
- `eprint!`: same as format! but the text is printed to the standard error (io::stderr).
- `eprintln!`: same as eprint!but a newline is appended.

[example](./formatting.rs)
