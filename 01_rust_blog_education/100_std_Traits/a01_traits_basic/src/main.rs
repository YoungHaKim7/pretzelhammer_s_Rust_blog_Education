trait Trait {
    // always return i32
    fn returns_num() -> i32;

    // returns implementation type
    fn returns_self() -> Self;
}

struct SomeType;
struct OtherType;

impl Trait for SomeType {
    fn returns_num() -> i32 {
        1
    }

    fn returns_self() -> Self {
        SomeType
    }
}

impl Trait for OtherType {
    fn returns_num() -> i32 {
        2
    }

    fn returns_self() -> Self {
        OtherType
    }
}

fn main() {
    println!("Hello, world!");
}
