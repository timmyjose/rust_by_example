type AliasedResult<T> = Result<T, std::num::ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str
        .parse::<i32>()
        .and_then(|f| second_number_str.parse::<i32>().map(|s| f * s))
}

fn print_aliased_result<T: std::fmt::Display>(res: AliasedResult<T>) {
    match res {
        Err(err) => println!("Got an error: {:?}", err),
        Ok(val) => println!("The result is {}", val),
    }
}

fn main() {
    let (a, b, c, d) = ("12", "13", "bar", "123");

    print_aliased_result(multiply(a, b));
    print_aliased_result(multiply(c, d));
}
