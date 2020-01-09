// main can also return a Result

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let bad_number_str = "123abc";

    let value = match bad_number_str.parse::<i32>() {
        Err(err) => return Err(err),
        Ok(val) => val,
    };

    println!("Parsed number = {}", value);

    Ok(())
}
