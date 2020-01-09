// The following are valid Rust macro designators:
// block, item, ident, tt, ty, pat, path, vis, stmt, expr, and literal

macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1 + 2);
    print_result!({
        let x = 100;
        let y = 200;
        let z = 300;

        x + y * z
    });
}
