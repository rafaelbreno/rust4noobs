## Primitives

### Summary
- [Scalar Types](#scalar-types)
- [Compound Types](#compound-types)
- [Mut keyword](#mut-keyword)

### Scalar Types
- signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
- unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
- floating point: `f32`, `f64`
- char Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
- bool either `true` or `false`
- and the unit type `()`, whose only possible value is an empty tuple: `()`

### Compound Types
- arrays like `[1, 2, 3]`
- tuples like `(1, true)`

### `mut` keyword
Variables in Rust are _immutable_ (value is constant), unless it's prefixed by `mut`
- Mutable variable types can be inferred
- __Cannot__ change it's type
```rust
// A mutable variable's value can be changed.
let mut mutable = 12; // Mutable `i32`
mutable = 21;

// Error! The type of a variable can't be changed.
mutable = true;

// Variables can be overwritten with shadowing.
let mutable = true;
```

### `const` keyword
- You aren’t allowed to use mut with constants
- They’re always immutable
- You declare constants using the `const` keyword
- The type of the value __MUST__ be annotated

### Example
```rust
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation

    let an_integer   = 5i32; // Suffix annotation
    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
}
```
