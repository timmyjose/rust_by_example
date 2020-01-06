fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            print!("FizzBuzz ");
        } else if n % 3 == 0 {
            print!("Fizz ");
        } else if n % 5 == 0 {
            print!("Buzz ");
        } else {
            print!("{} ", n);
        }
    }
    println!();

    iter_demo();
    iter_mut_demo();
    into_iter_demo();
}

fn iter_demo() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        print!("{} ", name);
    }
    println!();
}

fn iter_mut_demo() {
    let mut names = vec!["Bob".to_string(), "Frank".to_string(), "Ferris".to_string()];

    for name in names.iter_mut() {
        name.push('!');
    }

    for name in &names {
        print!("{} ", *name);
    }
    println!();
}

fn into_iter_demo() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        print!("{} ", name);
    }
    println!();
}
