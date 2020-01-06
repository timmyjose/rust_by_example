fn main() {
    let decimal = 84.1212_3423f32;

    let integer = decimal as u8;
    let character = integer as char;

    println!("{}, {}", integer, character);
}
