trait Trait {
    // always returns i32
    fn returns_num() -> i32;

    // returns implementing type
    fn returns_self() -> Self;
}

struct SomeType;
struct OtherType;

impl Trait for SomeType {
    fn returns_num() -> i32 {
        5
    }

    // Self == SomeType
    fn returns_self() -> Self {
        SomeType
    }
}

impl Trait for OtherType {
    fn returns_num() -> i32 {
        6
    }

    // Slef == OtherType
    fn returns_self() -> Self {
        OtherType
    }
}

// Fuctions
trait Default {
    // function
    fn default() -> Self;
}

fn main() {
    //the trait `Default` is not implemented for `i32
    // let zero: i32 = Default::default();

    let zero = i32::default();
    println!("zero {zero}");
}
