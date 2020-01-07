fn main() {
    let color = "green".to_string();

    let print = || println!("The colour is {}", color);
    print();

    let _immutable_borrow_color = &color;
    print();

    let _move_color = color;
    //print();
    //

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("Count = {}", count);
    };

    inc();
    inc();

    let _immutable_borrow_count = &count;
    //inc();

    let _move_count = count;
    //inc();

    let movable = Box::new(100);

    let consume = || {
        println!("Movable: {:?}", movable);
        std::mem::drop(movable);
    };

    consume();
    //consume();
}
