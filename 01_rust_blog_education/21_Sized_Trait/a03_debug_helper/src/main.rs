use std::fmt::Debug;

fn debug<T: Debug>(t: T) {
    println!("{:?}", t);
}
fn main() {
    debug("my str");
}
