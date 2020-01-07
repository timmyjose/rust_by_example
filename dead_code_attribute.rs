fn used_function() {
    println!("called used_function");
}

#[allow(dead_code)]
fn unused_function() {
    println!("called unused_function");
}

#[allow(dead_code)]
fn noisy_unused_function() {
    println!("called noisy_unused_function");
}

fn main() {
    used_function();
}
