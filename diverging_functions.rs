#![feature(never_type)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

fn foo() -> ! {
    panic!(
        "This is an example of a divergent function - a function that never returns to the caller."
    );
}

// not a divergent function
fn some_fn() {
    ()
}

fn sum_odd_numbers(upto: u32) -> u32 {
    let mut sum = 0;

    for i in 0..=upto {
        let val = match i % 2 {
            1 => i,
            _ => continue, // divergent function - castable to any type
        };

        sum += val;
    }

    sum
}

fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line");

    let s = sum_odd_numbers(100);
    println!("s = {}", s);

    let b: ! = foo();
    println!("This function never returns, and you will not see this line");
}
