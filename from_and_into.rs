use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {} }}", self.value)
    }
}

impl From<i32> for Number {
    fn from(term: i32) -> Self {
        Number { value: term }
    }
}

fn main() {
    let s = "Hello, world";
    let string = String::from(s); // the std::convert::From trait is in the Prelude

    assert_eq!(s, string);

    // custom from implementation
    let number = Number::from(100);
    println!("Number = {:#?}", number);
    println!("Number = {}", number);

    // the Into implementation comes for free
    let another_number: Number = 99.into();
    println!("another_number = {:#?}", another_number);
    println!("another_number = {}", another_number);
}
