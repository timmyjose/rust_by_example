#![allow(unused_variables)]

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "A" };
    let f = Droppable { name: "F" };

    {
        let b = Droppable { name: "B" };

        {
            let c = Droppable { name: "C" };
            std::mem::drop(f); // manual drop
        }
    }

    let d = Droppable { name: "D" };
    let e = Droppable { name: "E" };

    // order of drops - F, C, B, E, D, A
}
