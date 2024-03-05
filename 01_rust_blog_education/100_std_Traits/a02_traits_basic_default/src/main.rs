trait Default {
    // function
    fn default() -> Self;
}

impl Default for i32 {
    fn default() -> Self {
        0
    }
}

fn main() {
    let zero: i32 = Default::default();
    // let zero = i32::default();
    println!("zero: {}", zero);
}
