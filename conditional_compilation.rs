#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn not_on_linux() {
    println!("You are **not** on linux");
}

fn main() {
    not_on_linux();

    if cfg!(target_os = "linux") {
        println!("Yup, you're on linux");
    } else if cfg!(target_os = "darwin") {
        println!("You are on macOS!");
    } else {
        println!("You are on some other OS!");
    }
}
