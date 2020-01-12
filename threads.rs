use std::thread;

const NTHREADS: usize = 10;

fn main() {
    let mut handles = Vec::new();

    for i in 0..NTHREADS {
        handles.push(thread::spawn(move || {
            println!("This is thread {}", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
