// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute the command `rustlings hint vec2` if you need
// hints.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // This exact method is found in sec. 8.1 of rust lang book
        // N.B.: the * => dereferencing
        // Why dereference?
        // i is a mutable reference thus must first dereference
        // At the end of loop, mutable reference goes out of scope
        // Therefore, all changes are safe and allowed by borrowing rules
        // From: 8.3 on Hashmaps (paragraph after listing 8-26)
        *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
