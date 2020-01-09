// One use of impl Trait is to simplify complicated return types. If a function returns a type that
// implements a trait, then its return type can be simply impl Trait.

use std::iter;
use std::vec::IntoIter;

fn combine_vecs_explicit_return_type(
    u: Vec<i32>,
    v: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    u.into_iter().chain(v.into_iter()).cycle()
}

fn combine_vecs(u: Vec<i32>, v: Vec<i32>) -> impl iter::Iterator<Item = i32> {
    u.into_iter().chain(v.into_iter()).cycle()
}

// For some types, such as closures, impl Traits are requied (unless one uses Box)
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// similar to the first example, here is an example of a function that returns an iterator using
// map and filter such that its type cannot be written out since it internally uses closures
fn double_positives(numbers: &Vec<i32>) -> impl iter::Iterator<Item = i32> + '_ {
    numbers.iter().filter(|&&x| x > 0).map(|x| x * 2)
}

fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![6, 7, 8, 9, 10];

    for e in combine_vecs_explicit_return_type(vec1, vec2).take(20) {
        print!("{} ", e);
    }
    println!();

    let vec3 = vec![1, 2, 3, 4, 5];
    let vec4 = vec![6, 7, 8, 9, 10];

    for e in combine_vecs(vec3, vec4).take(20) {
        print!("{} ", e);
    }
    println!();

    let add10 = make_adder(10);
    for i in 0..=10 {
        print!("{} ", add10(i));
    }
    println!();

    let numbers = vec![-199, 1, -11, 0, -2, -1992, 3, 4, 5];
    let squares_positives = double_positives(&numbers).collect::<Vec<_>>();
    println!("{:?}", squares_positives);
}
