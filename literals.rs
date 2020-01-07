fn main() {
    let a = 1u8;
    let b = 1.2345f64;
    let c = 12345u128;

    println!("size of 'a' in bytes = {}", std::mem::size_of_val(&a));
    println!("size of 'b' in bytes = {}", std::mem::size_of_val(&b));
    println!("size of 'c' in bytes = {}", std::mem::size_of_val(&c));
}
