macro_rules! test {
    ($left: expr; and $right: expr) => {
        println!("{:?} and {:?} = {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right);
    };

    ($left: expr; or $right: expr) => {
        println!("{:?} or {:?} = {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left or $right);
    }
}

fn main() {
    test!(true; and true);
    test!(false; or true);
}
