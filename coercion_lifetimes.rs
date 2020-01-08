fn multiply<'a>(x: &'a i32, y: &'a i32) -> i32 {
    x * y
}

// a is at least as long b
fn choose_first<'a: 'b, 'b>(x: &'a i32, _: &'b i32) -> &'b i32 {
    x
}

fn main() {
    let first = 2;

    {
        let second = 3;

        println!("multiply: {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
