#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        println!("Destroying pair {:?}", self);
    }
}

fn main() {
    let p = Point::new(1.0, 2.3);
    println!("point = {:?}", p);

    let origin = Point::origin();
    println!("origin = {:?}", origin);

    let mut rect = Rectangle {
        p1: Point::new(0., 3.),
        p2: Point::new(4., 0.),
    };

    println!("Rectangle: {:#?}", rect);
    println!(
        "Area of rectangle = {}, perimeter = {}",
        rect.area(),
        rect.perimeter()
    );

    rect.translate(10., 20.);
    println!("Translated rectangle: {:#?}", rect);

    {
        let pair = Pair(Box::new(10), Box::new(20));
        pair.destroy();
    }
}
