fn main() {
    let mut counter = 0u32;

    let y = 'blk: loop {
        counter += 1;

        if counter == 10 {
            break 'blk counter * 2;
        }
    };

    assert_eq!(y, 20);
}
