fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yum yum!"),
        Some(thing) => println!("I like things! {} will suit me just fine.", thing),
        None => println!("You are a cruel person."),
    }
}

fn give_princess(gift: Option<&str>) {
    match gift {
        Some(thing) => {
            if thing == "snake" {
                panic!("Off with your head!");
            } else {
                println!("I love {}!", thing);
            }
        }
        None => panic!("Even a snake would have done!"),
    }
}

fn main() {
    let food = Some("cabbage");
    let snake = Some("snake");
    let nothing = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(nothing);

    let bird = Some("secretary");
    let no_thing = None;

    give_princess(bird);
    give_princess(Some("snake"));
    give_princess(no_thing);
}
