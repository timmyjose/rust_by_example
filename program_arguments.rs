fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    println!("Program name: {}", args[0]);
    if args.len() > 1 {
        let args = &args[1..];
        println!("I got {} arguments: {:?}", args.len(), args);
    }
}
