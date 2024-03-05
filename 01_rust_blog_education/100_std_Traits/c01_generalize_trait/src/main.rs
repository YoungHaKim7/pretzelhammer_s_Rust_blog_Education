trait Trait<'a, T> {
    fn new() -> Self;
    // signature uses generic type parameter
    fn func1(arg: T);

    // signature uses lifetime parameter
    fn func2(arg: &'a i32);

    // signature uses lifetime parameter and generic type parameter
    fn func3(arg: &'a T);
}

struct SomeType;

impl<'a> Trait<'a, i8> for SomeType {
    fn new() -> SomeType {
        SomeType
    }

    fn func1(arg: i8) {
        println!("i8: {}", arg);
    }

    fn func2(arg: &'a i32) {
        println!("&i32: {}", arg);
    }

    fn func3(arg: &'a i8) {
        println!("&i8: {}", arg);
    }
}

impl<'b> Trait<'b, u8> for SomeType {
    fn new() -> SomeType {
        SomeType
    }

    fn func1(arg: u8) {
        println!("i32: {}", arg);
    }

    fn func2(arg: &'b i32) {
        println!("&i32: {}", arg);
    }

    fn func3(arg: &'b u8) {
        println!("&i32: {}", arg);
    }
}

fn main() {
    // let mut some_type = SomeType::new();
    // SomeType::func1(1_i8);
    // SomeType::func2(&1);
    // SomeType::func3(&1_u8);
}
