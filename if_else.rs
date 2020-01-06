fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n == 9 {
        println!("{} is zero", n);
    } else {
        println!("{} is positive", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!("And is a small number... increas ten-fold");
        10 * n
    } else {
        println!("And is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
