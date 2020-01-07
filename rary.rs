//! rustc --crate-type=lib rary.rs
pub fn public_function() {
    println!("called rary's public function");
}

pub fn indirect_access() {
    println!("called rary's indirect access, which then ");
    private_function();
}

fn private_function() {
    println!("called rary's private function");
}
