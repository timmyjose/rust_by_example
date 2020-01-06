#![allow(irrefutable_let_patterns)]

enum Foo {
    Bar,
}

fn main() {
    let a = Foo::Bar;

    // this works even if the enum does not derive PartialEq because pattern matching relies on
    // the shape and types of the expressions and the values.
    if let Foo::Bar = a {
        println!("a is foobar!");
    }
}
