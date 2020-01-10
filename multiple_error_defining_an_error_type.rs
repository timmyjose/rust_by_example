use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct DoubleError; // custom error

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None // no source - this is the original source of error
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// custom Result type
type Result<T> = std::result::Result<T, DoubleError>;

fn double_first_element(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|e| e.parse::<i32>().map_err(|_| DoubleError).map(|e| e * 2))
}

fn print_result(res: Result<i32>) {
    match res {
        Ok(val) => println!("Result = {}", val),
        Err(err) => println!("Got an error: {}", err),
    }
}

fn main() {
    let numbers = vec!["11", "12", "13"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["foo", "bar", "quux"];

    print_result(double_first_element(numbers));
    print_result(double_first_element(empty));
    print_result(double_first_element(strings));
}
