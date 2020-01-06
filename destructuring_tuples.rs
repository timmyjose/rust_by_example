#![allow(unreachable_patterns)]

use std::io;
use std::str::FromStr;

fn get_number() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    i32::from_str(input.trim()).unwrap()
}

fn main() {
    let point = (get_number(), get_number());

    match point {
        (0, 0) => println!("This is the origin"),
        (x, 0) => println!("On the X-axis at a horizontal distance of {}", x),
        (0, y) => println!("On the Y-axis at a vertical height of {}", y),
        (x, y) => println!("A normal point: ({}, {})", x, y),
        _ => panic!("This is impossible and thereby redundant"),
    }
}
