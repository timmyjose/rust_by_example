struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let first_10_fibonacci_numbers = fibonacci().take(10).collect::<Vec<_>>();
    println!("{:?}", first_10_fibonacci_numbers);

    let mut sequence = 0..=5;

    while let Some(val) = sequence.next() {
        println!("{:?}", val);
    }

    for num in 0..=10 {
        print!("{} ", num);
    }
    println!();

    let sum_of_first_10_fibonacci_numbers = fibonacci().take(10).sum::<u32>();
    println!("{}", sum_of_first_10_fibonacci_numbers);

    for element in fibonacci().skip(10).take(5) {
        println!("{:10}", element);
    }

    let array = [11, 12, 13, 14, 15];
    for e in array.iter() {
        print!("{} ", e);
    }
    println!();
}
