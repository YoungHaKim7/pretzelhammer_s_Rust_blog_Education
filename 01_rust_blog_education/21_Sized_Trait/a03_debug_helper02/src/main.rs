use std::fmt::Debug;

fn debug<T: Debug + ?Sized>(t: &T) {
    println!("{:?}", t);
}
fn main() {
    debug("my str");
}
