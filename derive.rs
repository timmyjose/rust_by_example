#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.0 as f64 * 2.54)
    }
}

struct _Seconds;

fn main() {
    let foot = Inches(12);
    println!("one foot = {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigegr"
    };

    println!("one foot is {} than one meter", cmp);
}
