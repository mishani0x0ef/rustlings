// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit(u8),
    Echo(String),
    Move(MoveMessage),
    ChangeColor(u16, u16, u16),
}

fn main() {
    println!("{:?}", Message::Quit(1));
    println!("{:?}", Message::Echo(String::from("Hello")));
    println!("{:?}", Message::Move(MoveMessage { x: 1, y: 2 }));
    println!("{:?}", Message::ChangeColor(255, 255, 255));
}
