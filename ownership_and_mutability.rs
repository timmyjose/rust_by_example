/// Mutability can be changed, of course, when ownership changes
fn main() {
    let immutable_box = Box::new(99);
    println!("Immutable_box contains: {}", immutable_box);

    let mut mutable_box = immutable_box; // move
    *mutable_box += 1;
    println!("mutable_box contains: {}", mutable_box);
}
