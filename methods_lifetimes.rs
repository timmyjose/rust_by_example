struct Owner(i32);

impl Owner {
    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }

    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
}

fn main() {
    let mut owner = Owner(10);
    owner.print();
    owner.add_one();
    owner.print();
}
