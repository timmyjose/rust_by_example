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

    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "unicode codepoint {} is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "long strings can 
        span multiple
        lines, like
                literally";
    println!("{}", long_string);

    let raw_str = r"Escape sequences don't work here \x3F \u{211D}";
    println!("{}", raw_str);

    let quotes = r#"The is necessary if you want "quotes" inside a raw string"#;
    println!("{}", quotes);

    let another_quote =
        r######"You can add as many hash symbols along with r while defining a "raw string""######;
    println!("{}", another_quote);
}
