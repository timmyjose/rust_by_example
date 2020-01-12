use std::fs::File;
use std::io::Write;
use std::path::Path;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let displayable_path = path.display();

    let mut file = match File::create(&path) {
        Err(err) => panic!(
            "could not create file {}. Reason: {:?}",
            displayable_path, err
        ),
        Ok(handle) => handle,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(err) => panic!("Error while writing to file: {:?}", err),
        Ok(_) => println!("Finished writing to the file successfully"),
    }
}
