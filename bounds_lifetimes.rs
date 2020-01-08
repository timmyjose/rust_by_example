use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T: Debug>(x: &T) {
    println!("print: {:?}", x);
}

fn print_ref<'a, T>(x: &'a T)
where
    T: 'a + Debug,
{
    println!("print_ref: {:?}", x);
}

fn main() {
    let x = 7;
    let ref x_ref = x;

    print(x_ref);
    print_ref(x_ref);

    let y = 100;
    let y_ref = Ref(&y);

    print(&y_ref);
    print_ref(&y_ref);
}
