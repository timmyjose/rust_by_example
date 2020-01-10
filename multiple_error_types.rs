// note that the file will not run correctly - the next few files prefixed with
// multiple_error will list out various idiomatic strategies of handling Options
// with Results and/or handling Results with different error types.
fn double_first_element(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // possibly None
    2 * first.parse::<i32>().unwrap() // possible Err(ParseIntError)
}

fn main() {
    let numbers = vec!["11", "12", "13"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["foo", "bar", "baz"];

    println!("{}", double_first_element(numbers));
    println!("{}", double_first_element(empty));
    println!("{}", double_first_element(strings));
}
