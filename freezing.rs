#![allow(unused_variables)]

fn main() {
    let mut mutable_integer = 99;

    {
        let immutable_integer = &mutable_integer;
        //mutable_integer += 1;
        println!("Immutably borrowed {}", immutable_integer);
    }

    mutable_integer += 1;
    assert_eq!(mutable_integer, 100);
}
