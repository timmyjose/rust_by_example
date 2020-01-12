fn increase(x: i32) {
    println!("{}", x + 1);
}

fn decrease(x: i32) {
    println!("{}", x - 1);
}

fn help() {
    println!("Usage: 
             match_args <string> - check whether the string is the answer
             match_args increase|decrease <integer> - increase or decrease the given integer by one");
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    match args.len() {
        0 => help(),
        1 => match args[0].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => println!("This is not the answer"),
        },

        2 => {
            let cmd = &args[0];
            let number = &args[1];

            let number = match number.parse::<i32>() {
                Err(_) => {
                    eprintln!("not a number!");
                    help();
                    return;
                }
                Ok(n) => n,
            };

            match cmd.as_str() {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("no such command");
                    help();
                }
            }
        }
        _ => help(),
    }
}
