#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(ref c) => println!("User pressed {}", c),
        WebEvent::Paste(ref s) => println!("User pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("User clicked co-ordinates ({}, {})", x, y),
    }
}

fn main() {
    // enums are very useful in simulatign state machines!
    let mut event = WebEvent::PageLoad;
    inspect(event);
    event = WebEvent::KeyPress('x');
    inspect(event);
    event = WebEvent::Click { x: -100, y: 200 };
    inspect(event);
    event = WebEvent::Paste(String::from("Hello, world!"));
    inspect(event);
    event = WebEvent::PageUnload;
    inspect(event);
}
