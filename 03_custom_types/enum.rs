enum Action {
    // Those are unit structs
    Create,
    Update,
    Delete,
    // An example of a C-like struct
    Read{ id: String },
}

fn inspect(a: Action) {
    match a {
        Action::Create => println!("Action Create"),
        Action::Read { id } => println!("Action Read from id '{}'", id),
        Action::Update => println!("Action Update"),
        Action::Delete => println!("Action Delete"),
    }
    
}

fn main() {
    let c = Action::Create;
    let r = Action::Read{ id: String::from("user-id-1") };
    let u = Action::Update;
    let d = Action::Delete;

    inspect(c);
    inspect(r);
    inspect(d);
    inspect(u);
}
