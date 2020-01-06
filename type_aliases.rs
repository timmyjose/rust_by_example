enum VeryLongEnumNameForAAVerySimpleSetOfOperations {
    Add,
    Subtract,
}

impl VeryLongEnumNameForAAVerySimpleSetOfOperations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryLongEnumNameForAAVerySimpleSetOfOperations;

fn main() {
    let mut op = Operations::Add;
    println!("{}", op.run(1, 2));

    op = Operations::Subtract;
    println!("{}", op.run(1, 2));
}
