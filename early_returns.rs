// a more imperative approach

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first = match first_number_str.parse::<i32>() {
        Err(err) => return Err(err),
        Ok(f) => f,
    };

    let second = match second_number_str.parse::<i32>() {
        Err(err) => return Err(err),
        Ok(s) => s,
    };

    Ok(first * second)
}

fn print_result(res: Result<i32, ParseIntError>) {
    match res {
        Err(err) => println!("Got an error: {:?}", err),
        Ok(val) => println!("The result is {}", val),
    }
}

fn main() {
    let (a, b, c, d) = ("12", "13", "123", "quux");

    print_result(multiply(a, b));
    print_result(multiply(c, d));
}
