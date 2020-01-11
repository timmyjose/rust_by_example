fn checked_division(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn try_division(x: i32, y: i32) {
    match checked_division(x, y) {
        Some(val) => println!("{}", val),
        None => println!("No result"),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);
    println!(
        "unwrapping {:?} gives {}",
        optional_float,
        optional_float.unwrap()
    );
    println!("unwrapping {:?} will panic {}", none, none.unwrap());
}
