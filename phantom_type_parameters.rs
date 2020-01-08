//! Phantom types are marker types which have special meaning at compile-time.

#![allow(unused_variables)]
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(Debug, PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

fn main() {
    let tuple1: PhantomTuple<char, f32> = PhantomTuple('x', PhantomData);
    println!("tuple1 = {:?}", tuple1);

    let tuple2: PhantomTuple<char, f32> = PhantomTuple('x', PhantomData);
    println!("tuple2 = {:?}", tuple2);

    let tuple3: PhantomTuple<char, f64> = PhantomTuple('x', PhantomData);
    println!("tuple3 = {:?}", tuple3);

    println!("is {:?} == {:?}? {}", tuple1, tuple2, tuple1 == tuple2);
    //println!("is {:?} == {:?}? {}", tuple1, tuple3, tuple1 == tuple3);

    let struct1: PhantomStruct<bool, i32> = PhantomStruct {
        first: true,
        phantom: PhantomData,
    };
    let struct2: PhantomStruct<bool, i32> = PhantomStruct {
        first: true,
        phantom: PhantomData,
    };
    let struct3: PhantomStruct<bool, String> = PhantomStruct {
        first: true,
        phantom: PhantomData,
    };

    println!("Is {:?} == {:?}? {}", struct1, struct2, struct1 == struct2);
    //println!("Is {:?} == {:?}? {}", struct2, struct3, struct2 == struct3);
}
