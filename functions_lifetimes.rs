fn print_one<'a>(x: &'a i32) {
    println!("print_one resul is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b, 'c>(x: &'a i32, y: &'b i32, z: &'c i32) {
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

fn main() {
    let x = 7;
    let mut y = 10;
    let z = 11;

    print_one(&x);
    print_multi(&x, &y, &z);
    add_one(&mut y);
    print_one(&y);

    let a = pass_x(&z, &x);
    println!("a = {}, z = {}", a, z);
}
