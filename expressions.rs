#![allow(unused_must_use)]

use std::io;
use std::str::FromStr;

fn get_pair() -> (i32, i32) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let v = input
        .trim()
        .split_whitespace()
        .map(|e| i32::from_str(e).unwrap())
        .collect::<Vec<_>>();
    (v[0], v[1])
}

fn main() {
    let x = 5u32;

    let y = {
        let z = 100u32;
        x + z
    };

    let a = {
        1 + 2;
    };

    assert_eq!(x, 5);
    assert_eq!(y, 105);
    assert_eq!(a, ());

    let complicated_expression_value = {
        let pair = get_pair();
        pair.0 + pair.1
    };

    println!(
        "complicated_expression_value = {}",
        complicated_expression_value
    );
}
