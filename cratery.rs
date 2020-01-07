//! The crate_type and crate_name atributes do not work with Cargo.
//! They only work with rustc.

#![crate_type = "lib"]
#![crate_name = "quux"]

pub fn public_function() {
    println!("called public_function");
}

fn private_function() {
    println!("called private_function");
}

pub fn indirect_access() {
    println!("called indirect_access, which then ");
    private_function();
}
