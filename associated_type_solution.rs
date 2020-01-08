#[derive(Debug)]
struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &i32, b: &i32) -> bool {
        self.0 == *a && self.1 == *b
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let c1 = Container(1, 2);
    println!(
        "Does {:?} contain {} and {}? {}",
        c1,
        &1,
        &2,
        c1.contains(&1, &2)
    );
    println!(
        "First number = {}, second number = {}",
        c1.first(),
        c1.last()
    );
    println!("Difference = {}", difference(&c1));
}
