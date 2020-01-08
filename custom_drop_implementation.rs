#![allow(unused_variables)]
#[derive(Debug)]
struct ToDrop(i32);

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.0);
    }
}

fn main() {
    let d1 = ToDrop(1);

    {
        for id in 2..=10 {
            let _ = ToDrop(id);
        }
    }

    let d2 = ToDrop(11);
}
