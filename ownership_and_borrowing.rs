fn eat_box_i32(b: Box<i32>) {
    println!("Destroying box that contains: {}", b);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(1);
    let stacked_i32 = 2i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let ref_to_i32 = &boxed_i32;
        borrow_i32(ref_to_i32);
        //eat_box_i32(boxed_i32);
        //borrow_i32(&boxed_i32);
    }

    eat_box_i32(boxed_i32);
}
