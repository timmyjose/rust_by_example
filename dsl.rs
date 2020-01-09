macro_rules! calculate {
    (eval $x: expr) => {
        let e: usize = $x;
        println!("{:?} = {:?}", stringify!($x), e);
    };
}

fn main() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 12) / (4 / 2)
    }

    calculate![eval 1 * 2 + (4 / (1 + 1))];
}
