fn main() {
    println!("{} days", 31);

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "dog",
        verb = "bites",
        subject = "man"
    );

    // special additional formatting
    println!(
        "There are {:b} kinds of people in the world. People who know binary, and those who don't!",
        2
    );

    // text-alignment
    println!("{number:>width$}", number = 1, width = 10);
    println!("{number:<width$}", number = 2, width = 10);

    // padding with zeroes
    println!("{number:>0width$}", number = 1, width = 10);

    // format strings are checked at compile-time
    println!("My name is {0}. {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    println!("{:?}", Structure(99));

    println!("Pi is approximately {:.3}", std::f64::consts::PI);
}
