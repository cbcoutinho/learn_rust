enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let x: Message = Message::Move { x: 3, y: 4 };

    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    let m = Message::Write("Hello, world".to_string());

    println!("My Message: {}", m.Write());
}
