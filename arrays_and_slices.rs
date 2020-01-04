fn analyse_slice(slice: &[i32]) {
    println!("First elememt of the slice = {}", slice[0]);
    println!("Length of the slice = {}", slice.len());
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    analyse_slice(&array);
    analyse_slice(&array[2..4]);

    let pixels = [0u8; 1000];
    assert_eq!(pixels.len(), 1000);

    // arrays are always stack-allocated
    println!("Size of array = {} bytes", std::mem::size_of_val(&array));
}