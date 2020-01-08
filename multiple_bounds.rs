use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug print: {:?}", t);
    println!("Display print: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t = {:?}", t);
    println!("u = {:?}", u);
}

fn main() {
    let string = "words";
    let vector = vec![1, 2, 3, 4, 5];
    let array = [1, 2, 3, 4, 5];

    compare_prints(&string);
    compare_types(&vector, &string);
    //compare_prints(&vector);
    compare_types(&vector, &array);
}
