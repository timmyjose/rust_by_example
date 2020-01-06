static mut NUMBER_OF_ITEMS: i32 = 0;

fn eventually_none_sequence() -> Option<i32> {
    unsafe {
        if NUMBER_OF_ITEMS != 0 {
            let val = NUMBER_OF_ITEMS;
            NUMBER_OF_ITEMS -= 1;
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    unsafe {
        NUMBER_OF_ITEMS = 3;
    }

    while let Some(val) = eventually_none_sequence() {
        println!("Got {}", val);
    }

    unsafe {
        NUMBER_OF_ITEMS = 5;
    }

    loop {
        match eventually_none_sequence() {
            Some(val) => println!("Got {}", val),
            None => break,
        }
    }
}
