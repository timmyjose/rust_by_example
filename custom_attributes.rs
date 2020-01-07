#[cfg(some_condition)]
fn conditional_function() {
    println!("called conditional_function");
}

// rustc custom_attributes.rs --cfg some_condition
fn main() {
    conditional_function();
}