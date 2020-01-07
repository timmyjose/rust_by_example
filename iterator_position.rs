fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let index_of_first_even_number = vec1.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(1));

    let index_of_first_negative_number = vec2.iter().position(|x| x < &0);
    assert_eq!(index_of_first_negative_number, None);
}
