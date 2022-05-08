## Custom Types

### Summary
- [Struct](#struct)
- [Enum](#enum)
- [Enum C-like](#enum-c-like)
- [Constant](#constant)

### Struct
There are three types of structures ("structs") that can be created using the struct keyword:
- `Tuple` structs, which are, basically, named `tuples`.
- The classic _C structs_
- `Unit` structs, which are field-less, are useful for _generics_.

```rust
// Tuple Struct
struct Pair(i32, i32);

// Tuple with 3 fields
struct Triple(i32, i32, i32);

// Classic struct
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

fn main() {
    // Defining the fields
    let name: String = String::from("Rafael");
    let age: u8 = 22;
    
    // Assigning the fields into a Struct
    let rafael: Person = Person{
        name: name,
        age: age,
    };

    // Accessing the fields with var.field_name
    println!("Person name: {}", rafael.name);
    println!("Person age: {}", rafael.age);

    // using the `..` it will use the fields from 
    // another struct, in this case `rafael`,
    // inheriting the field `age`
    let breno: Person = Person { name: String::from("Breno"), ..rafael };

    // Accessing the fields with var.field_name
    println!("Person name: {}", breno.name);
    println!("Person age: {}", breno.age);

    // Tuple structs can be set using
    // the same syntax of tuples.
    let p: Pair = Pair(1,2);
    println!("Pair contains: {} and {}", p.0, p.1);

    // Or it can be set using the named fields
    // from 0..N , N being the number of field minus 1.
    let t: Triple = Triple{
        0: 1,
        1: 2,
        2: 3,
    };
    println!("Triple contains: {}, {} and {}", t.0, t.1, t.2);

    // TODO:
    //   - add Unit example

}
```

### Enum
- The `enum` keyword allows the creation of a type which may be one of a few different _variants_. 
- Any variant which is `valid` as a struct is also valid as an `enum`.
- Also there's the `use` keyword
  - The `use` declaration can be used so manual scoping isn't needed

```rust
enum Action {
    // Those are unit structs
    Create,
    Update,
    Delete,
    // An example of a C-like struct
    Read{ id: String },
}

enum Quality {
    Poor,
    Medium,
    High,
}

fn inspect_action(a: Action) {
    match a {
        Action::Create => println!("Action Create"),
        Action::Read { id } => println!("Action Read from id '{}'", id),
        Action::Update => println!("Action Update"),
        Action::Delete => println!("Action Delete"),
    }
    
}

fn inspect_quality(q: Quality) {
    match q {
        Quality::Poor   => println!("Quality Poor"),
        Quality::Medium => println!("Quality Medium"),
        Quality::High   => println!("Quality High"),
    }
}

fn main() {
    let c = Action::Create;
    let r = Action::Read{ id: String::from("user-id-1") };
    let u = Action::Update;
    let d = Action::Delete;

    inspect_action(c);
    inspect_action(r);
    inspect_action(d);
    inspect_action(u);


    // Using the keyword `use`, it allows
    // you to refer to the Enum field directly
    // instead of using Quality::Poor to access it.
    use crate::Quality::{Poor, Medium, High};

    let p = Poor;
    let m = Medium;
    let h = High;

    inspect_quality(p);
    inspect_quality(m);
    inspect_quality(h);
}
```

### Enum C-like
- Enums also can be use as [C-like](https://en.wikipedia.org/wiki/Enumerated_type#C)

```rust
enum Number {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn main() {
    println!("{}", Number::Zero as i32);
    println!("{}", Number::One as i32);
    println!("{}", Number::Two as i32);
}
```

### Consts and Static
Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:
- `const`: An unchangeable value (the common case).
- `static`: A possibly mutable variable with `'static` lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable `static` variable is `>>unsafe<<`.

```rust
// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
```
