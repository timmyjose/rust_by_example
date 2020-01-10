fn main() {
    // string literals reside in a binary's read-only memory (usually the BSS section of an
    // executable)
    let pangram = "the quick brown fox jumps over the lazy dog";
    println!("pangram: {:?}", pangram);

    println!("the pangram in reverse...");
    for word in pangram.split_whitespace().rev() {
        print!("{} ", word);
    }
    println!();

    let mut chars = pangram.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("{:?}", string);

    let chars_to_trim = &[' ', ','];
    let trimmed_str = string.trim_matches::<&[_]>(chars_to_trim); // bizarre
    println!("{:?}", trimmed_str);

    // heap-allocated string
    let alice = String::from("I like dogs");
    let bob = alice.replace("dogs", "cats");

    println!("Alice says: {:?}", alice);
    println!("Bob says: {:?}", bob);
}
