#![allow(dead_code)]
// C-style structs
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u32,
}

// unit structs
struct Nil;

// tuple structs
struct Pair(i32, i32);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(
    Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    }: &Rectangle,
) -> f32 {
    (x2 - x1) * (y1 - y2)
}

fn square(p: Point, delta: f32) -> Rectangle {
    Rectangle {
        top_left: p,
        bottom_right: Point { x: p.x + delta, y: p.y - delta },
    }
}

fn main() {
    let name = "Bob";
    let age = 42;
    let bob = Person { name, age };
    println!("{:?}", bob);

    let point = Point { x: 1.12, y: -0.002 };
    println!("x-coordinate: {}, y-coordinate: {}", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("bottom_right = {:?}", bottom_right);

    let Point {
        x: top_edge,
        y: bottom_edge,
    } = point;
    assert_eq!(top_edge, 1.12);
    assert_eq!(bottom_edge, -0.002);

    let rectangle = Rectangle {
        top_left: Point {
            x: top_edge,
            y: bottom_edge,
        },
        bottom_right: bottom_right,
    };
    println!("{:#?}", rectangle);

    let pair = Pair(100, -1899);
    assert_eq!(pair.0, 100);
    assert_eq!(pair.1, -1899);

    let Pair(x, y) = pair;
    assert!(x == pair.0);
    assert!(y == pair.1);

    let rectangle = Rectangle {
        top_left: Point { x: 0., y: 3. },
        bottom_right: Point { x: 4., y: 0. },
    };
    assert_eq!(rect_area(&rectangle), 12.0);

    let square = square(Point { x: 0., y: 3. }, 4.);
    assert_eq!(rect_area(&square), 16.);
}
