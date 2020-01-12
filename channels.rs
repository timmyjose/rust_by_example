use std::sync::mpsc;
use std::thread;

const NTHREADS: usize = 8;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for id in 0..NTHREADS {
        let tx = tx.clone();

        handles.push(thread::spawn(move || {
            tx.send(id).unwrap();
            println!("Thread {} finished", id);
        }));
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);

    for _ in 0..NTHREADS {
        ids.push(rx.recv().unwrap());
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", ids);
}
