use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl List {
    fn new() -> Self {
        List(Vec::new())
    }

    fn add(&mut self, val: i32) {
        self.0.push(val);
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for (idx, val) in self.0.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", idx, *val)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let mut nums = List::new();

    for num in 1..=10 {
        nums.add(num);
    }

    println!("{:?}", nums);
    println!("{}", nums);

    let empty_nums = List::new();
    println!("{:?}", empty_nums);
    println!("{}", empty_nums);

    let singleton_list = List(vec![1]);
    println!("{:?}", singleton_list);
    println!("{}", singleton_list);
}
