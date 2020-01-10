use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

type Result<T> = std::result::Result<T, DoubleError>;

impl error::Error for DoubleError {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            DoubleError::Parse(ref err) => Some(err), // underlying cause is the parsing error
        }
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element!"),
            DoubleError::Parse(ref err) => err.fmt(f),
        }
    }
}

// this is so that ? works correctly while wrapping a ParseIntError into a DoubleError instance
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first_element(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print_result(res: Result<i32>) {
    match res {
        Ok(val) => println!("Result = {}", val),
        Err(err) => println!("Got error: {}", err),
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
