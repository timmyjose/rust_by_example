fn elided_input(x: &i32) {
    println!("elided input = {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input = {}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let x = 100;

    elided_input(&x);
    annotated_input(&x);

    let x_ref = elided_pass(&x);
    println!("x_ref = {}", x_ref);

    let another_x_ref = annotated_pass(&x);
    println!("another_x_ref = {}", another_x_ref);
}
