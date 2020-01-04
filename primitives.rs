#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let b: bool = true;
    assert!(b);

    let a_float: f64 = 1.2345;
    let an_int = 5i32;

    let mut inferred_type = 12;
    inferred_type = 99999999991235555i64;

    let mut mutable = 12;
    mutable = 12345;

    // shadowing is perfectly legal in Rust
    let mutable = false;
    assert!(!mutable);
}
