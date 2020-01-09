#![allow(deprecated)]

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first = try!(first_number_str.parse::<i32>());
    let second = try!(second_number_str.parse::<i32>());

    Ok(first * second)
}

fn print_result(res: Result<i32, ParseIntError>) {
    match res {
        Err(err) => println!("Got an error: {:?}", err),
        Ok(val) => println!("The result is {}", val),
    }
}

fn main() {
    let (a, b, c, d) = ("12", "13", "foo", "123");

    print_result(multiply(a, b));
    print_result(multiply(c, d));
}
