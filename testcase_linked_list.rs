use crate::List::*;

#[derive(Debug)]
enum List {
    Nil,
    Cons(u32, Box<List>),
}

impl List {
    fn new() -> Self {
        Nil
    }

    fn prepend(self, elem: u32) -> Self {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> usize {
        match *self {
            Nil => 0,
            Cons(_, ref node) => 1 + node.len(),
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Nil => "Nil".to_string(),
            Cons(elem, ref node) => format!("{} {}", elem, node.stringify()),
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);
    list = list.prepend(5);

    assert_eq!(list.len(), 5);
    println!("{}", list.stringify());
}
