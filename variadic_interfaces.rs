macro_rules! calculate {
    (eval $x: expr) => {
        let e: usize = $x;
        println!("{:?} = {}",stringify!($x), e);
    };

    (eval $x: expr, $(eval $y: expr),+) => {
        calculate! { eval $x }
        calculate! { $(eval $y),+ }
    };
}

fn main() {
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
