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
