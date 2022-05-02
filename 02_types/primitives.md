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

### Array
- Array can be declared as `let slice: [i32; 5] = [1,2,3,4,5];`, where:
  - `let` - the _keyword_ to declare a expression
  - `slice` - the name
  - `:` - type identifyer
  - `[T: L]` - to define it's a array of type `T` and length of `L`
  - `=` - declare that the next expression it's their value
  - `[1,2,3,4,5]` = the value
- You can access an element of position `N` using:
  - `slice[N]` - will access the value of position `N`
  - If the length of _slice_ is less than `N`, it won't compile.

```rust
fn main() {
    let slice: [i32; 5] = [1,2,3,4,5];
    
    println!("{:?}", slice[0]);
}
```

### Tuple
- A tuple is a collection of values of different types. 
- Tuples are constructed using parentheses `()`, and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. 
- Functions can use tuples to return multiple values, as tuples can hold any number of values.
- You can access an element of position `N` using:
  - `tuple.N` - will access the value of position `N`
  - If the length of _tuple_ is less than `N`, it won't compile.

```rust
fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("First element: {}", long_tuple.0);
    println!("Second element: {}", long_tuple.1);
}
```
