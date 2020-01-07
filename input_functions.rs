fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("Hello, I am function!");
}

fn main() {
    let closure = || println!("Hello, I am closure!");

    call_me(function);
    call_me(closure);
}
