#[derive(Debug)]
struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, f: &i32, s: &i32) -> bool {
        self.0 == *f && self.1 == *s
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let c1 = Container(1, 2);
    println!(
        "Does container {:?} contains {} and {}? {}",
        c1,
        &1,
        &2,
        c1.contains(&1, &2)
    );
    println!("First number: {}, second number: {}", c1.first(), c1.last());
    println!("Difference = {}", difference(&c1));
}
