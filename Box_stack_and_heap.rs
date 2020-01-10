use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn main() {
    // stack allocated variables
    let point = origin();
    let rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // heap-allocated rectangle
    let boxed_rectangle = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_point = Box::new(origin());
    let box_in_a_box = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of::<Point>()
    );

    // this should be the same as above
    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );

    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed rectangle's type occupies {} bytes",
        mem::size_of::<Rectangle>()
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    let unboxed_point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}
