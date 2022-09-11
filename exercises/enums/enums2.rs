// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


struct Anything{
    x: u8,
    y:u8,
}

#[derive(Debug)]
enum Message {
    Echo(String),
    Quit,
    ChangeColor(u8,u8,u8),
    Move { x: i32, y: i32 },
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
