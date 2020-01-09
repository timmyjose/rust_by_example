fn give_princess(gift: &'static str) {
    if gift == "snake" {
        panic!("Argggh!!!");
    }

    println!("Thank you for the {}", gift);
}

fn main() {
    give_princess("teddy-bear");
    give_princess("snake");
}
