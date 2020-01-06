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
    let pair = get_pair();

    match pair {
        (x, y) if x == y => println!("Got twins!"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("First number of pair is odd"),
        _ => println!("No correlation!"),
    }
}
