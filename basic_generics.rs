#[derive(Debug)]
struct A;

#[derive(Debug)]
struct Singleton(A);

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    let a = A;
    println!("{:?}", a);

    let s = Singleton(a);
    println!("{:?}", s);

    let sgi = SingleGen(100);
    println!("{:?}", sgi);

    let sgs = SingleGen("Hello, world");
    println!("{:?}", sgs);

    let sga = SingleGen(A);
    println!("{:?}", sga);
}
