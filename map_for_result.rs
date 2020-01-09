// the map combinator can also be used with Result types

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Err(err) => Err(err),
        Ok(f) => match second_number_str.parse::<i32>() {
            Err(err) => Err(err),
            Ok(s) => Ok(f * s),
        },
    }
}

fn better_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str
        .parse::<i32>()
        .and_then(|f| second_number_str.parse::<i32>().map(|s| f * s))
}

fn print_result(r: Result<i32, ParseIntError>) {
    match r {
        Err(err) => println!("Got an error: {:?}", err),
        Ok(val) => println!("Result = {}", val),
    }
}

fn main() {
    let (a, b, c, d) = ("12", "13", "foo", "223");

    let p1 = multiply(a, b);
    print_result(p1);

    let p2 = multiply(c, d);
    print_result(p2);

    let p3 = better_multiply(a, b);
    print_result(p3);

    let p4 = better_multiply(c, d);
    print_result(p4);
}
