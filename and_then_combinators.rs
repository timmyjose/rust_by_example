#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug, Clone, Copy)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn eat(food: Food, day: Day) {
    match cookable_v1(food) {
        None => println!("Am I a joke to you on {:?}?", day),
        Some(food) => println!("I love {:?} on {:?}", food, day),
    }
}

fn eat1(food: Food, day: Day) {
    match cookable_v1(food) {
        None => println!("Am I a joke to you on {:?}?", day),
        Some(food) => println!("I love {:?} on {:?}", food, day),
    }
}

// note that the map combinator returns Option<Option<T>> whereas
// the and_then combinator works directly with Option<T>
// Almost sort of how Applicative works in Haskell
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday); // None
    eat(cordon_bleu, Day::Tuesday); // None
    eat(cordon_bleu, Day::Wednesday); // None
    eat(steak, Day::Monday); //Some(Food::Steak)
    eat(steak, Day::Tuesday); //Some(Food::Steak)
    eat(steak, Day::Wednesday); //Some(Food::Steak)
    eat(sushi, Day::Monday); // None
    eat(sushi, Day::Tuesday); // None
    eat(sushi, Day::Wednesday); // None

    eat1(cordon_bleu, Day::Monday); // None
    eat1(cordon_bleu, Day::Tuesday); // None
    eat1(cordon_bleu, Day::Wednesday); // None
    eat1(steak, Day::Monday); //Some(Food::Steak)
    eat1(steak, Day::Tuesday); //Some(Food::Steak)
    eat1(steak, Day::Wednesday); //Some(Food::Steak)
    eat1(sushi, Day::Monday); // None
    eat1(sushi, Day::Tuesday); // None
    eat1(sushi, Day::Wednesday); // None
}
