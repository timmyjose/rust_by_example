#[derive(Debug, Copy, Clone)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Nil;
    let nil_copy = nil;

    println!("{:?}, {:?}", nil, nil_copy);

    let p1 = Pair(Box::new(1), Box::new(2));
    let p2 = p1;
    //println!("{:?}", p1);
    println!("{:?}", p2);

    let p3 = p2.clone();
    println!("{:?}, {:?}", p2, p3);

    std::mem::drop(p2);
    println!("{:?}", p3);
}
