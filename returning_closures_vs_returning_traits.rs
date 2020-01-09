fn return_add_one() -> impl FnOnce(i32) -> i32 {
    |x| x + 1
}

fn return_add_two() -> Box<dyn FnOnce(i32) -> i32> {
    Box::new(|x| x + 2)
}

struct Foo {}
struct Bar {}

trait Quux {
    fn name(&self) -> &'static str;
}

impl Quux for Foo {
    fn name(&self) -> &'static str {
        "Foo"
    }
}

impl Quux for Bar {
    fn name(&self) -> &'static str {
        "Bar"
    }
}

fn return_some_quux(b: bool) -> Box<dyn Quux> {
    if b {
        Box::new(Foo {})
    } else {
        Box::new(Bar {})
    }
}

fn main() {
    let add_one = return_add_one();
    println!("{}", add_one(100));

    let add_two = return_add_two();
    println!("{}", add_two(100));

    let foo = return_some_quux(true);
    println!("{}", foo.name());

    let bar = return_some_quux(false);
    println!("{}", bar.name());
}
