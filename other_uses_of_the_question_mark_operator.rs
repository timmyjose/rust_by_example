use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first_element(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
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
