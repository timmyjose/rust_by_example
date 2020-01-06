fn main() {
    let mut counter = 1u32;

    while counter <= 100 {
        if counter % 15 == 0 {
            println!("FizzBuzz");
        } else if counter % 3 == 0 {
            println!("Fizz");
        } else if counter % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", counter);
        }

        counter += 1;
    }
}
