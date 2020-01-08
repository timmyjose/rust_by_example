use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

#[allow(dead_code)]
struct Triangle {
    lenght: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn main() {
    let rect = Rectangle {
        height: 1.23,
        length: 2.345,
    };
    println!("{:?}, {:?}", rect.area(), area(&rect));
    print_debug(&rect);
}
