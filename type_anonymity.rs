fn apply<F: FnOnce()>(f: F) {
    f();
}

fn main() {
    let x = 7;
    let print = || println!("x is {}", x);

    apply(print);
}
