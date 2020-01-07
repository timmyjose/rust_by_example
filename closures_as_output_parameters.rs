fn create_fn() -> impl Fn() {
    let name = "Fn".to_owned();

    move || println!("This is {}", name)
}

fn old_create_fn() -> Box<dyn Fn()> {
    let name = "Old Fn".to_owned();

    Box::new(move || {
        println!("This is {}", name);
    })
}

#[allow(bare_trait_objects)]
fn even_older_create_fn() -> Box<Fn()> {
    let name = "Even older Fn".to_owned();

    Box::new(move || {
        println!("This is {}", name);
    })
}

fn create_fnmut() -> impl FnMut() {
    let name = "FnMut".to_owned();

    move || println!("This is {}", name)
}

fn create_fnonce() -> impl FnOnce() {
    let name = "FnOnce".to_owned();

    move || println!("This is {}", name)
}

fn main() {
    let fn_plain = create_fn();
    let old_fn_plain = old_create_fn();
    let even_older_fn_plain = even_older_create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    old_fn_plain();
    even_older_fn_plain();
    fn_mut();
    fn_once();
}
