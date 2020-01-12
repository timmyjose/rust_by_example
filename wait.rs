use std::process::Command;

fn main() {
    let mut cmd = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = cmd.wait().unwrap();

    println!("Adios from main!");
}
