struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "Baaa!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "Mooo!"
    }
}

// Note that the impl Trait idiom does not work here - it only works while returning closures,
// which are basically anonymous types. In this case, structs are not anonymous types.
// Note, however, that this Box<dyn Trait> approach works for closures as well.
// See: returning_closures_vs_returning_traits
fn random_animal(randomness_factor: f64) -> Box<dyn Animal> {
    if randomness_factor < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let mut randomness_factor = 0.4123;
    let animal = random_animal(randomness_factor);
    println!("{:?}", animal.noise());

    randomness_factor = 0.678;
    let animal = random_animal(randomness_factor);
    println!("{:?}", animal.noise());
}
