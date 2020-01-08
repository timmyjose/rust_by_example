#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Borrowed { x: &10 }
    }
}

fn main() {
    let borrowed = Borrowed::default();
    assert_eq!(*borrowed.x, 10);
}
