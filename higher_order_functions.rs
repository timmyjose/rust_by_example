fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");

    let mut sum = 0;

    for i in 0.. {
        if i * i > 1000 {
            break;
        }

        if is_odd(i * i) {
            sum += i * i;
        }
    }

    println!("sum = {}", sum);

    let sum1 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared <= 1000)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n| acc + n);

    println!("sum1 = {}", sum1);

    assert_eq!(sum, sum1);
}
