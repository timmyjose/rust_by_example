fn main() {
    fizzbuzz_to(100);
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzBuzz");
    } else if is_divisible_by(n, 3) {
        println!("Fizz");
    } else if is_divisible_by(n, 5) {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
}

fn is_divisible_by(n: u32, m: u32) -> bool {
    m != 0 && n % m == 0
}

fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}
