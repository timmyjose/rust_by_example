//! The recommended way to implement ToString support for custom types is to
//! implement std::fmt::Display, which automatically provides a toString implementation
//! in addition to making the type printable.

use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 10f32 };
    println!("{}", circle);

    let parsed_number: i16 = "12345".parse().unwrap();
    assert_eq!(parsed_number, 12345);

    let another_parsed_number = "100".parse::<u8>().unwrap();
    assert_eq!(another_parsed_number, 100);
}
