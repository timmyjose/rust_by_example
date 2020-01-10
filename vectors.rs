fn main() {
    // iterators can be collected into vectors
    let collected_iterator = (0..10).collect::<Vec<i32>>();
    println!("collected_iterator = {:?}", collected_iterator);

    // vec! to initialise a vector
    let mut xs = vec![11, 12, 13];
    println!("xs = {:?}", xs);

    println!("Pushing 14");
    xs.push(14);
    println!("xs = {:?}", xs);

    println!(
        "vector lenght = {}, capacity = {}, second element = {}",
        xs.len(),
        xs.capacity(),
        xs[1]
    );

    println!("Poppint the last element: {:?}", xs.pop());
    println!("xs = {:?}", xs);

    println!("Contents of xs...");
    for e in &xs {
        print!("{} ", *e);
    }
    println!();

    for (idx, val) in xs.iter().enumerate() {
        println!("In position {}, we have element {}", idx, val);
    }

    for e in xs.iter_mut() {
        *e += 100;
    }

    for e in &xs {
        print!("{} ", *e);
    }
    println!();

    for e in &mut xs {
        *e -= 100;
    }

    for e in xs.iter() {
        print!("{} ", e);
    }
    println!();
}
