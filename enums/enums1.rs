// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    // See 6.1 - Defining an Enum 
    Quit, 
    Echo, 
    Move, 
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
