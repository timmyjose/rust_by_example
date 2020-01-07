fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {:?}", greeting);

        farewell.push_str("!");
        println!("Then I screamed {:?}", farewell);
        println!("Now, I can sleep! ZZzzzzzzz");

        std::mem::drop(farewell);
    };

    diary();

    let double = |x| x * 2;
    println!("3 doubled = {}", apply_to_3(double));

    apply(|| println!("Hola, Mundo!"));
}
