use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Inch;

#[derive(Debug, Copy, Clone)]
struct Mm;

#[derive(Debug, Copy, Clone)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_metre: Length<Mm> = Length(100.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_metres = one_metre + one_metre;

    println!("two_feet = {:?}", two_feet);
    println!("two_metres = {:?}", two_metres);

    //let one_foot_plus_one_metre = one_foot + one_metre;
}
