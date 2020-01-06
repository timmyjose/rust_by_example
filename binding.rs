fn age() -> usize {
    42
}

fn maybe_some_number() -> Option<i32> {
    Some(32)
}

fn main() {
    match age() {
        0 => println!("I am not born yet!"),
        n @ 1..=12 => println!("A wee child of age {}", n),
        n @ 13..=19 => println!("A rowdy teenager with the rowdy age of {}", n),
        n => println!("I am an old person of age {}", n),
    }

    match maybe_some_number() {
        Some(n @ 32) => println!("The answer to Life, the Universe, and Everything is {}!", n),
        Some(n) => println!("Some random number {}", n),
        None => println!("Not today, Darkseid!"),
    }
}
