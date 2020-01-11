use std::collections::HashMap;

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1234");
    contacts.insert("Ashley", "123-34556");
    contacts.insert("Katie", "43120-12121");
    contacts.insert("Robert", "123-121221");

    match contacts.get(&"Daniella") {
        Some(number) => println!("Calling Daniella: {}", call(number)),
        _ => println!("Don't have Daniella's number"),
    }

    match contacts.get(&"Daniel") {
        Some(number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number"),
    }

    contacts.insert("Daniel", "998-18181");

    match contacts.get(&"Daniel") {
        Some(number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number"),
    }

    match contacts.get(&"Ashley") {
        Some(number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number"),
    }

    contacts.remove(&"Ashley");

    for (contact, number) in contacts.iter() {
        println!("{}: {}", contact, number);
    }
}

fn call(number: &str) -> &str {
    match number {
        "798-1234" => "Hola, Daniel! We are watching you!",
        "123-34556" => "Hello, Ashley! Don't forget to look under the bed!",
        _ => "Howdy, stranger!",
    }
}
