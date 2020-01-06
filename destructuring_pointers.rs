fn main() {
    let reference = &100;

    match reference {
        &val => println!("Got a value: {}", val),
    }

    match *reference {
        val => println!("Again, got a value: {}", val),
    }

    // interesting - create a reference where none exists
    let ref this_is_a_reference = 100;

    match this_is_a_reference {
        &val => println!("Yet again, got a value: {}", val),
    }

    let mut some_value = 100;

    match some_value {
        ref rval => println!("Once more, got a value: {}", rval), // more accurately, *rval
    }

    match some_value {
        ref mut rval => {
            *rval += 100;
            println!("Finally, got a value: {}", *rval);
        }
    }
}
