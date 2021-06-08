// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)


mod sausage_factory {
    // Need to make function public before it can be called 
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
