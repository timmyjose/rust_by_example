#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut bob = Person {
        name: "Bob".to_string(),
        age: 42,
    };

    match bob {
        ref bob_ref => println!("{:#?}", bob_ref),
    }

    let ref mut another_bob_ref = bob;
    another_bob_ref.name.push_str("bers");

    match bob {
        ref bob_ref => println!("{:?}", *bob_ref),
    }

    println!("{:?}", bob);
}
