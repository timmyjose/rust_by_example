struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_t: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_t: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let bluejay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A bluejay is {}", blue(&bluejay));
}
