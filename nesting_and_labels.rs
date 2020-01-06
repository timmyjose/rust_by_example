#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This print statenent will never be executed");
    }

    println!("Outside all loops");
}
