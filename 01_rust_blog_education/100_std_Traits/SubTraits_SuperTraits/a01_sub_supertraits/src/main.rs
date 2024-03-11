trait Supertrait {
    fn method(&self) {
        println!("in supertrait");
    }
}

trait Subtrait: Supertrait {
    // this looks it might impl or
    // override Supertrait::method but it
    // does not
    fn method(&self) {
        println!("in subtrait");
    }
}

struct SomeType;

impl Supertrait for SomeType {}

impl Subtrait for SomeType {}

fn main() {
    // SomeType.method(); // X ambiguous method call
    <SomeType as Supertrait>::method(&SomeType);
    <SomeType as Subtrait>::method(&SomeType);
}
