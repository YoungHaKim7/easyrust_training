struct QuitMesage; // unit struct

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct

struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("print : {:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
