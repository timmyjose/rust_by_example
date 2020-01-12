use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!(
            "could not open file {} for reading. Reason: {:?}",
            display, err
        ),
        Ok(handle) => handle,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => panic!(
            "error while reading file {} contents. Reason: {:?}",
            display, err
        ),
        Ok(_) => println!("{}", contents),
    }
}
