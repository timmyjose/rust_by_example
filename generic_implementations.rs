#[derive(Debug)]
struct Val {
    val: f64,
}

struct GenericVal<T> {
    gen_val: T,
}

// non-generic impl
impl Val {
    fn value(&self) -> f64 {
        self.val
    }
}

// generic impl
impl<T> GenericVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let val = Val { val: 1.2345 };
    println!("{}", val.value());

    let gval1 = GenericVal { gen_val: 1.2345 };
    println!("{:?}", gval1.value());

    let gval2 = GenericVal {
        gen_val: "Hola, Mundo",
    };
    println!("{:?}", gval2.value());

    let gval3 = GenericVal {
        gen_val: Val { val: 1.2345 },
    };
    println!("{:?}", gval3.value());
}
