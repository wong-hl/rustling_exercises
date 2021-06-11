// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // Function for the method of the trait needs to be the same
    // as the above declaration in trait AppendBar
    //
    // self needs to be mutable to add a string literal
    fn append_bar(mut self) -> Self {
        // push_str is the function in Stuct String that allows
        // for the addition of a string literal
        self.push_str("Bar");
        // Need to return self
        // Firstly, as that's in the definition
        // Secondly, to prevent the String from falling out of scope
        // as ownership of it has been taken (I think)
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
