use std::process::Command;

fn main() {
    let cmd = Command::new("zig")
        .arg("version")
        .output()
        .unwrap_or_else(|err| panic!("{:?}", err));

    if cmd.status.success() {
        println!("{}", String::from_utf8_lossy(&cmd.stdout));
    } else {
        println!("{}", String::from_utf8_lossy(&cmd.stderr));
    }
}
