#![allow(dead_code)]

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);
    println!("Two is {}", Number::Two as i32);

    println!("Red = {:06x}", Color::Red as i32);
    println!("Green = {:06x}", Color::Green as i32);
    println!("Blue = {:06x}", Color::Blue as i32);
}
