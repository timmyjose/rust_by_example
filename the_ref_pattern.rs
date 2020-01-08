#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';
    let ref c_r1 = c;
    let c_r2 = &c;

    println!("c_r1 == c_r2? {}", *c_r1 == *c_r2);

    let point = Point { x: 0, y: 0 };

    let copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    assert_eq!(copy_of_x, 0);

    let mut mutable_point = point; // copy

    {
        let Point {
            x: _,
            y: ref mut ref_mut_y,
        } = mutable_point;
        *ref_mut_y += 100;
    }

    assert_eq!(mutable_point.y, 100);
    println!("point is {:?}, mutable_point is {:?}", point, mutable_point);

    let mut mutable_tuple = (Box::new(100), -200);
    {
        let (_, ref mut last) = mutable_tuple;
        *last += 300;
    }

    assert_eq!(*mutable_tuple.0, 100);
    assert_eq!(mutable_tuple.1, 100);
}
