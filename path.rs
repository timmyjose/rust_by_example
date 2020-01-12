use std::path::Path;

fn main() {
    let current_directory = Path::new(".");
    let displayable_path = current_directory.display();
    println!("{}", displayable_path);

    let new_path = current_directory.join("foo").join("bar");
    match new_path.to_str() {
        None => panic!("Not a valid UTF-8 string"),
        Some(val) => println!("new path is {}", val),
    }
}
