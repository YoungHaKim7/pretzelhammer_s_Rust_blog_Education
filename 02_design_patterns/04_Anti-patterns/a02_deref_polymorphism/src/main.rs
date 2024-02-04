use std::ops::Deref;

struct Foo {}
struct Bar {
    f: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &self.f
    }
}

impl Foo {
    fn m(&self) {
        println!("Foo print");
    }
}

fn main() {
    let b = Bar { f: Foo {} };
    b.m();
    println!("Hello, world!");
}
