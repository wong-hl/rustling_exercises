// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)


mod delicious_snacks {
    // Original: use self::fruits etc
    // Need to make this usage public so that the "API" of fruits and veggie 
    // can be called directly
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
