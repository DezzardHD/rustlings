#[derive(Debug)]
enum Message {
    Resize = 1,
    Move = 2,
    Echo = 3,
    ChangeColor = 4,
    Quit = 5,
    // TODO: Define a few types of messages as used below.
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
