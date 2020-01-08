#![allow(dead_code)]

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

struct NonPrintable;

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    vec.print_in_option();

    let string = String::from("Hello, world");
    string.print_in_option();

    let array = [1, 2, 3, 4, 5];
    array.print_in_option();

    //let non_printable = NonPrintable;
    //non_printable.print_in_option();
}
