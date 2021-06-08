// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // Just need to declare it as a String type instead of a
    // string slice/literal (&str)
    let blue = String::from("blue");
    //"blue"
    blue
}
