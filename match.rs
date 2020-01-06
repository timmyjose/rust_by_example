use std::io;
use std::str::FromStr;

fn get_number() -> u32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    u32::from_str(input.trim()).unwrap()
}

fn main() {
    let n = get_number();

    match n {
        1 => println!("Got a one"),
        2 | 3 | 4 | 5 => println!("This is prime!"),
        13..=19 => println!("A teen!"),
        _ => println!("ain't special!"),
    }

    let b = true;
    let result = match b {
        false => 0,
        true => 1,
    };

    assert_eq!(result, 1);
}
