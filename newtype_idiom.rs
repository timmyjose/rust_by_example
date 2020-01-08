/// the newtype idiom

#[derive(Debug)]
struct Years(i64);

#[derive(Debug)]
struct Days(i64);

impl Years {
    fn to_days(&self) -> i64 {
        self.0 * 365
    }
}

impl Days {
    fn to_years(&self) -> i64 {
        self.0 / 365
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(18);
    println!(
        "{:?} is days = {}. Old enough? {}",
        age,
        age.to_days(),
        old_enough(&age)
    );

    let days = Days(8978);
    println!("{:?} = {} years", days, days.to_years());
}
