// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // Obvious
    string_slice("blue");

    // to_string => resulting one will be a String
    string("red".to_string());

    // Obvious
    string(String::from("hi"));

    // to_owned => will be String
    string("rust is fun!".to_owned());

    // into => String
    string("nice weather".into());

    // format! => string
    string(format!("Interpolation {}", "Station"));

    // [0..1] => slice. Thus, string slice
    string_slice(&String::from("abc")[0..1]);

    // Not sure on this one but result indicates this is a string slice
    // Trim probably is equivalent to a slice ¯\_ (ツ)_/¯
    string_slice("  hello there ".trim());

    // to_string => String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));

    // to_lowercase => String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
