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
