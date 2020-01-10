use std::num::ParseIntError;

fn double_first_element1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|e| e.parse::<i32>())
}

fn double_first_element2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|e| e.parse::<i32>());
    let opt = opt.map_or(Ok(None), |r| r.map(Some))?;

    Ok(opt)
}

fn main() {
    let numbers1 = vec!["11", "12", "13"];
    let empty1: Vec<&str> = vec![];
    let strings1 = vec!["foo", "bar", "baz"];

    println!("{:?}", double_first_element1(numbers1));
    println!("{:?}", double_first_element1(empty1));
    println!("{:?}", double_first_element1(strings1));

    let numbers2 = vec!["11", "12", "13"];
    let empty2: Vec<&str> = vec![];
    let strings2 = vec!["foo", "bar", "baz"];

    println!("{:?}", double_first_element2(numbers2));
    println!("{:?}", double_first_element2(empty2));
    println!("{:?}", double_first_element2(strings2));
}
