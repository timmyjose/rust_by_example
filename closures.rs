fn main() {
    fn inc(x: i32) -> i32 {
        x + 1
    }
    assert_eq!(inc(99), 100);

    let inc1 = |x: i32| -> i32 { x + 1 };
    assert_eq!(inc1(99), 100);

    let inc2 = |x| x + 1;
    assert_eq!(inc2(99), 100);

    let closure_returning_one = || 1;
    assert_eq!(closure_returning_one(), 1);
}
