fn main() {
    let bytestring: &[u8; 21] = b"This is a byte string";
    println!("bytestring: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74";
    println!("{:?}", escaped);

    let raw_bytestring = br"]u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
        println!("{}", my_str);
    } else {
        println!("not a valid string");
    }

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // non UTF-8

    match std::str::from_utf8(shift_jis) {
        Ok(my_str) => println!("conversion successful: {}", my_str),
        Err(err) => println!("conversion failed. Error: {:?}", err),
    }
}
