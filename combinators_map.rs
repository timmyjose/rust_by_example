#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Orange,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(f) => Some(Peeled(f)),
        None => None,
    }
}

fn chop(food: Option<Peeled>) -> Option<Chopped> {
    match food {
        Some(Peeled(f)) => Some(Chopped(f)),
        None => None,
    }
}

fn cook(food: Option<Chopped>) -> Option<Cooked> {
    match food {
        Some(Chopped(f)) => Some(Cooked(f)),
        None => None,
    }
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(f) => println!("I love {:?}", f),
        None => println!("No food today!"),
    }
}

// map here is a combinator for the Option type,
// this is quite similar to a Monadic (in the Haskell sense) way of handling effects
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn main() {
    let cooked_carrots = cook(chop(peel(Some(Food::Carrot))));
    eat(cooked_carrots);

    let cooked_apple = process(Some(Food::Apple));
    eat(cooked_apple);

    let no_potato = process(None);
    eat(no_potato);
}
