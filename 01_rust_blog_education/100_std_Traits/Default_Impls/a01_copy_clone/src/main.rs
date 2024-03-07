trait Trait {
    fn method(&self) {
        println!("default(Trait) impl");
    }
}

struct SomeType;
struct OtherType;

impl Trait for SomeType {}

impl Trait for OtherType {
    fn method(&self) {
        println!("OtherType impl");
    }
}

fn main() {
    SomeType.method();
    OtherType.method();
}
