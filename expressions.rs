#![allow(unused_must_use)]

fn main() {
    let x = 5u32;

    let y = {
        let z = 100u32;
        x + z
    };

    let a = {
        1 + 2;
    };

    assert_eq!(x, 5);
    assert_eq!(y, 105);
    assert_eq!(a, ());
}
