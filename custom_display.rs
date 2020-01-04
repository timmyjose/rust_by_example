use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

fn main() {
    let min_max = MinMax(-1, 213);
    println!("{:?}", min_max);
    println!("{}", min_max);

    let (big_range, small_range) = (MinMax(-10, 10), MinMax(-100, 1000));
    println!(
        "The small range is {small} and the big range is {big}",
        small = small_range,
        big = big_range
    );
    println!(
        "The small range is {small:?} and the big range is {big:?}",
        small = small_range,
        big = big_range
    );

    let point = Point2D {
        x: -199.0,
        y: 28.233,
    };
    println!("{:?}", point);
    println!("{}", point);

    let c = Complex { re: 3.3, im: 7.2 };
    println!("{:?}", c);
    println!("{}", c);
}
