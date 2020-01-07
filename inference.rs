fn main() {
    let elem = 255u8;
    let mut vec = Vec::new();
    vec.push(elem);

    assert_eq!(vec[0], elem);
    assert_eq!(std::mem::size_of_val(&vec[0]), std::mem::size_of::<u8>());
}
