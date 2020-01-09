trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u32;
}

struct Form {
    username: String,
    age: u32,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u32 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "Bob".to_owned(),
        age: 42,
    };

    println!("Username = {}", <Form as UsernameWidget>::get(&form));
    println!("Username = {}", UsernameWidget::get(&form));
    println!("Age = {}", <Form as AgeWidget>::get(&form));
    println!("Age = {}", AgeWidget::get(&form));
}
