static NUM: i32 = 100;

// this works since 'static is the longest possible lifetime
// and longer lifetimes can be coerced into shorter lifetimes
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "Hola, Mundo";
        println!("static_string = {:?}", static_string);
    }

    {
        let lifetime_num = 200;
        println!("coerced_static = {}", coerce_static(&lifetime_num));
    }

    println!("NUM = {}", NUM);
}
