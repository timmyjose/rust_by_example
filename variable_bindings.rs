fn main() {
    let an_integer = 1u32;
    let a_bool = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("copied_integer = {:?}", copied_integer);
    println!("a_bool = {}", a_bool);
    println!("unit = {:?}", unit);

    let _unused_variable = 100;
    let _noisy_integer = 2u32;
}
