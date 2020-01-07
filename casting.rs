#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.122345f32;
    let integer = decimal as u8;
    let character = integer as char;

    println!("{}, {}, {}", decimal, integer, character);

    println!("1000 as a u16 is {}", 1000 as u16);

    // whenever casting to an unsigned type T, std::T::MAX + 1 is added/subtracted until
    // the value fits within T.
    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as u8 is {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 = {}", (-1i8) as u8);

    // same as (1000 as u16)
    println!("1000 mod 256 = {}", 1000 % 256);
    println!("128 as i16 is {}", 128 as i16);
    println!("128 as i8 is {}", 128 as i8);
    println!("232 as i8 is {}", 232 as i8);
}
