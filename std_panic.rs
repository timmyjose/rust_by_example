fn division(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        panic!("attempting division by zero");
    }

    x / y
}

fn main() {
    let x = Box::new(0f64);

    division(100.0, *x);
}
