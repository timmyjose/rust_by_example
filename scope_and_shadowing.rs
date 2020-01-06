fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        let long_lived_binding = 100;

        println!("short binding = {}", short_lived_binding);
        println!("inner long binding = {}", long_lived_binding);
    }

    println!("long binding = {}", long_lived_binding);

    let long_lived_binding = "Hello, world";
    println!("shadowed long binding = {}", long_lived_binding);
}
