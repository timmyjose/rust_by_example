fn destroy_box(b: Box<i32>) {
    println!("Destroying a box that contains {}", b);
}

fn main() {
    let x = 5i32;
    let y = x; // copy, not move

    println!("x = {}, y = {}", x, y);

    let a = Box::new(100);
    println!("box contents: {}", a);

    let b = a; // move
    destroy_box(b);
}
