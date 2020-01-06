#[derive(Debug)]
enum Foo {
    Bar,
    Baz,
    Quux(u32),
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(val) = number {
        println!("Matched {}", val);
    }

    match number {
        Some(val) => println!("Again, matched some value {}", val),
        None => {}
    }

    if let Some(c) = letter {
        println!("Got a letter {}", c);
    } else {
        println!("Found no letter");
    }

    let i_like_letters = false;

    if let Some(e) = emoticon {
        println!("Matched {:?}", e);
    } else if i_like_letters {
        println!("Didn't find any numbers. Let's go with letters");
    } else {
        println!("Didn't find any letters either. Let's go with some random emoticon");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Quux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Quux(_value) = c {
        println!("c is one hundred!");
    }

    if let Foo::Quux(_value @ 200) = c {
        println!("This line will not be printed");
    }

    if let Foo::Quux(value @ 100) = c {
        println!("c = {}", value);
    }
}
