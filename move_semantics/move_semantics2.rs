// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0 = Vec::new();

    // Original Error message: is that a move of vec0 here and the subsequent
    // computation of len in line 22 required a borrow
    //
    // I think because of fill_vec function, ownership of vec0 was transferred
    // to vec1 (output of function) so vec0 is no longer in the scope. Hence,
    // it does not exist anymore. So when it is being called in line 22, the
    // length can no longer be found. Hence, throwing an error.
    // Therefore, to prevent it from leaving the scope, a clone of vec0 needs
    // to be passed to the function such that vec0 still persists in main
    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
