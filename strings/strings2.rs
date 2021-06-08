// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    let word = String::from("green"); // Try not changing this line :)

    // Need to pass reference to not lose ownership?
    // NO! according to the hint (and I actually forgot this after reading
    // the book) BUT it is actually just need to COERCE the String to be
    // a &str as declared in the function
    //
    // How could I forget coersion? I remember thinking that's such a
    // good word choice for what is going on
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
