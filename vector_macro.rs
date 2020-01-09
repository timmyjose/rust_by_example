macro_rules! my_vec {
    ($($x: expr),*) => {{
        {
            let mut v = vec![];

            $(v.push($x);)*
            v
        }
    }}
}

fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    let empty_vec: Vec<&str> = my_vec![];
    assert_eq!(empty_vec.len(), 0);
}
