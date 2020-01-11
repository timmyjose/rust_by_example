use std::collections::HashSet;

fn main() {
    let mut a = vec![1, 2, 3].into_iter().collect::<HashSet<i32>>();
    let mut b = vec![2, 3, 4, 5].into_iter().collect::<HashSet<i32>>();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    //assert!(b.insert(4), "HashSet already contains element");

    b.insert(5);

    println!("{:?}", a);
    println!("{:?}", b);

    println!("union = {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!(
        "intersection = {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );
    println!("difference = {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!(
        "symmetric difference = {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
