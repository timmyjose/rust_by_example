#[allow(dead_code)]
struct Unprintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u32,
}

fn main() {
    println!("{:?}", DebugPrintable(100));
    println!("{:?}", Deep(DebugPrintable(101)));

    // formatting specifiers are the same as for fmt::Display
    println!("There are {:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?}'s name'",
        "Slater",
        "Christian",
        actor = "actor"
    );

    // pretty-printing
    let bob = Person {
        name: "Bob",
        age: 42,
    };
    println!("{:?}", bob);
    println!("{:#?}", bob);
}
