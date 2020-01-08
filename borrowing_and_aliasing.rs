#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 1, y: 2, z: 3 };
    let borrowed_point = &point;
    let another_borrowed_point = &point;

    println!(
        "borrowed_point = {:?}, another_borrowed_point = {:?}",
        borrowed_point, another_borrowed_point
    );

    let mutable_borrow = &mut point;

    mutable_borrow.x += 100;
    mutable_borrow.y += 100;
    mutable_borrow.z += 100;

    let new_borrowed_point = &point;

    println!("new_borrowed_point = {:?}", new_borrowed_point);
}
