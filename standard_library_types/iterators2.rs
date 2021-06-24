// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    // "save" the output of the match
    let mut a: String = match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    };
    // Collate the remaining chars as a string, then append it to a
    // This changes it in place => need an additional step to return
    // the full initial string
    a.push_str(&c.collect::<String>());
    a
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // Initialise output vector of strings
    let mut output: Vec<String> = Vec::new();
    // Iteterate over all items in words
    // For a given word, capitalise the first and add it to the output
    for item in words.iter() {
        output.push(capitalize_first(item));
    }
    output
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // Initalise output string
    let mut output: String = String::new();
    // Iteterate over all items in words
    // For a given word, capitalise the first and append it to output string
    for item in words.iter() {
        output.push_str(&capitalize_first(item));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
