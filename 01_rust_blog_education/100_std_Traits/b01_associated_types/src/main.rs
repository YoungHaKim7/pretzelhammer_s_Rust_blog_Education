trait Trait {
    type AssociatedType;
    fn func(arg: Self::AssociatedType);
}

// ZST
struct SomeType;
struct OtherType;

impl Trait for SomeType {
    type AssociatedType = i8; // chooses i8
    fn func(arg: Self::AssociatedType) {
        println!("{arg}");
    }
}

impl Trait for OtherType {
    type AssociatedType = u8; // chooses u8
    fn func(arg: Self::AssociatedType) {
        println!("{arg}");
    }
}

fn main() {
    dbg!(SomeType::func(-1_i8));
    dbg!(OtherType::func(1_u8));
}
