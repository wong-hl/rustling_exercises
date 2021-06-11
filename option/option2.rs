// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // Make this an if let statement whose value is "Some" type
    //
    // Syntax takes a pattern and an expression separated by
    // an equal sign. Works in teh same way as match
    // BUT loses the exhaustive checking of match
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    //
    // While let allows it it iterate over the entire vector
    //
    //    while let Some(int) = optional_integers_vec.pop() {
    //        // If let extracts the value that was put inside the vector
    //        // of options
    //        if let Some(integer) = int {
    //            println!("current value: {}", integer);
    //        }
    //    }

    // Let's try stacking this
    // THIS WORKS!
    // THAT IS FUCKING COOL
    //while let Some(Some(integer)) = optional_integers_vec.pop() {
    //    println!("current value: {}", integer);
    //}

    // Had a look at the hint, apparently there is also
    // Option::flatten
    // According to the docts, this converts from
    // Option<Option<T>> to Option<T>
    while let Some(integer) = optional_integers_vec.pop().flatten() {
        println!("current value: {}", integer);
    }
}
