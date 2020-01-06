#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Poor => println!("The poor have no money..."),
        Rich => println!("The rich have lots of money..."),
    }

    match work {
        Civilian => println!("Civilians stay at home"),
        Soldier => println!("Soldiers go to war"),
    }
}
