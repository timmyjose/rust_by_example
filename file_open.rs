use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("dry.rs");
    let displayable_path = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open file {}. Error: {:?}", displayable_path, err),
        Ok(handle) => handle,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => panic!("couldn't read file contents. Reason: {:?}", err),
        Ok(_) => println!("Contents of file {}:\n\n{}", displayable_path, contents),
    };
}
