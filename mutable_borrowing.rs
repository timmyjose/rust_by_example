#![allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Book {
    name: &'static str,
    author: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {:?}", book);
}

fn new_edition(book: &mut Book) {
    book.year += 1;
    println!("I mutably borrowed {:?}", book);
}

fn main() {
    let immutabook = Book {
        name: "Mathemagical Themas",
        author: "Douglas Hofstadter",
        year: 1979,
    };

    let mut mutabook = immutabook; // note that this is a copy
    borrow_book(&immutabook);
    borrow_book(&mutabook);
    //new_edition(&mut immutabook);
    new_edition(&mut mutabook);
    new_edition(&mut mutabook);
}
