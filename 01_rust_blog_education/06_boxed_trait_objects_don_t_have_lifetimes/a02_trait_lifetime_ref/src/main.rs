trait Trait {}

struct Struct {}

struct Ref<'a, T>(&'a T);

impl Trait for Struct {}
impl Trait for &Struct {} // impl Trait directly on a ref type
impl<'a, T> Trait for Ref<'a, T> {} // impl Trait on a type containg refs

fn main() {
    println!("Hello, world!");
}
