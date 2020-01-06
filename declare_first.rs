fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a_binding = {}", a_binding);

    let another_binding;
    //println!("another_binding = {}", another_binding);
    another_binding = 100;
    println!("another_binding = {}", another_binding);
}
