#![allow(unused_variables)]
use std::fmt;

// destructuring of parameters works as expected
fn reverse((x, y): (i32, bool)) -> (bool, i32) {
    (y, x)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(Matrix(a, b, c, d): Matrix) -> Matrix {
    Matrix(a, c, b, d)
}

fn main() {
    let pair = (100, false);
    let (bb, ii) = reverse(pair);
    assert_eq!(bb, false);
    assert_eq!(ii, 100);

    let matrix = Matrix(1.0, 2.3, 3.2, -1.12);
    println!("{:?}", matrix);
    println!("{}", matrix);

    let transposed_matrix = transpose(matrix);
    println!("{}", transposed_matrix);

    println!("One element tuple: {:?}", (5u64,));
    println!("Just an integer: {:?}", (5u64));

    // destructuring of tuples
    let tuple = (1, "hello", true, 1.233, Matrix(0.12, -1.122, 92.2, -2.2));
    let (a, b, c, d, e) = tuple;
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    // long tuple cannot be printed!
    let very_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("{:?}", very_long_tuple);
}
