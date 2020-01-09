struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("Already sheared");
        } else {
            println!("Shearing... done");
            self.naked = true;
        }
    }
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "baaaa!"
    }
}

struct Cat {
    fluffy: bool,
    name: &'static str,
}

impl Cat {
    fn is_fluffy(&self) -> bool {
        self.fluffy
    }

    fn make_fluffy(&mut self) {
        if self.is_fluffy() {
            println!("Already fluffy");
        } else {
            println!("Making fluffy... done");
            self.fluffy = true;
        }
    }
}

impl Animal for Cat {
    fn new(name: &'static str) -> Cat {
        Cat {
            fluffy: false,
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "Meow!"
    }

    fn talk(&self) {
        println!("My cat does not deign to talk to common folk");
    }
}

fn main() {
    let mut dolly = Sheep::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
    dolly.shear();

    let mut pickles = Cat::new("Pickles");
    pickles.talk();
    Animal::talk(&pickles); // UFCS
    pickles.make_fluffy();
    pickles.make_fluffy();
    pickles.talk();
    assert_eq!(pickles.name(), "Pickles");

    let mut bob: Sheep = Animal::new("Bob");
    bob.shear();
    bob.shear();
    bob.talk();
}
