fn main() {
    let mut count = 0u32;

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Okay, that's enough");
            break;
        }
    }
}
