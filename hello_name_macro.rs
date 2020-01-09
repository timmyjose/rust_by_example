macro_rules! say_hello_to {
    ($name: expr) => {
        println!("Hello, {}", $name);
    };
}

fn main() {
    say_hello_to!("Timmy");
    say_hello_to!("Jose");
}
