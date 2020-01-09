#![feature(const_fn)]

const fn double_x(x: i32) -> i32 {
    x * 2
}

//fn double_x(x: i32) -> i32 {
//    x * 2
//}

const FIVE: i32 = 5;
const TEN: i32 = double_x(FIVE);

fn main() {
    println!("{}", TEN);
}
