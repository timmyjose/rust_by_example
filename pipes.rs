use std::io::{Read, Write};
use std::process::{Command, Stdio};

static PANGRAM: &str = "the quick brown fox jumps over the lazy dog";

fn main() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(err) => panic!("Error while trying to open pipe: {:?}", err),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(err) => panic!("Error while piping output ot stdin: {:?}", err),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut output = String::new();
    match process.stdout.unwrap().read_to_string(&mut output) {
        Err(err) => panic!("Error while reading wc output: {:?}", err),
        Ok(_) => println!("wc output: {}", output),
    }
}
