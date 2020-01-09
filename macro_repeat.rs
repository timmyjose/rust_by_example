macro_rules! find_min {
    ($x: expr) => { $x };

    ($x: expr, $($y: expr),+) => {
       std::cmp::min($x, find_min!($($y),+))
    }
}

macro_rules! find_max {
    ($x: expr) => { $x };

    ($x: expr, $($y: expr),+) => {
        std::cmp::max($x, find_max!($($y),+))
    }
}

fn main() {
    assert_eq!(find_min!(1, 2, 3), 1);
    assert_eq!(find_min!(-1), -1);
    assert_eq!(find_max!(1, 2, 3), 3);
    assert_eq!(find_max!(-1), -1);
}
