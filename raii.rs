/// The memory model in Rust is essentially compiler-enforced RAII.

fn create_box() {
    let b = Box::new(100); // allocated on the heap
    println!("b = {}", b);
    // b gets de-allocated here
}

fn main() {
    let box1 = Box::new(1);

    {
        let box2 = Box::new(2);
        println!("box 2 = {}", box2);
        // box2 gets de-allocated here
    }

    for _ in 0..100 {
        create_box();
    }

    let box3 = Box::new(3);
    println!("box 3 = {}", box3);
    println!("box 1 = {}", box1);

    // box3 and then box1 get de-allocated here.
}
