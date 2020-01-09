fn main() {
    let a = "12";
    let b = "13";
    println!("{}", multiply(a, b));

    let c = "12";
    let d = "foo";
    println!("{}", multiply(c, d));
}

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}
