extern crate rary;

// rustc executable.rs --extern rary=library.rlib
fn main() {
    rary::public_function();
    rary::indirect_access();
}
