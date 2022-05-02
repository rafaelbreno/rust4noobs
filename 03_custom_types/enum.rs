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
