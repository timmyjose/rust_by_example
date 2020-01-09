trait Person {
    fn name(&self) -> &'static str;
}

trait Student: Person {
    fn university(&self) -> &'static str;
}

trait Programmer {
    fn favourite_language(&self) -> &'static str;
}

trait CompSciStudent: Student + Programmer {
    fn git_username(&self) -> &'static str;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!("Hello, my name is {}. I study at {} university, my favourite programming language is {}, and my Git username is {}",
            student.name(),
            student.university(),
            student.favourite_language(),
            student.git_username())
}

struct Individual {
    name: &'static str,
    age: u32,
    university: &'static str,
    favourite_language: &'static str,
    git_username: &'static str,
}

impl Individual {
    fn new(
        name: &'static str,
        age: u32,
        university: &'static str,
        favourite_language: &'static str,
        git_username: &'static str,
    ) -> Self {
        Individual {
            name,
            age,
            university,
            favourite_language,
            git_username,
        }
    }

    fn age(&self) -> u32 {
        self.age
    }
}

impl Person for Individual {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Student for Individual {
    fn university(&self) -> &'static str {
        self.university
    }
}

impl Programmer for Individual {
    fn favourite_language(&self) -> &'static str {
        self.favourite_language
    }
}

impl CompSciStudent for Individual {
    fn git_username(&self) -> &'static str {
        self.git_username
    }
}

fn main() {
    let bob = Individual::new("Bob", 21, "FooBar", "OHAI", "quux");
    println!("{}", comp_sci_student_greeting(&bob));
    println!("My age is {}", bob.age());
}
