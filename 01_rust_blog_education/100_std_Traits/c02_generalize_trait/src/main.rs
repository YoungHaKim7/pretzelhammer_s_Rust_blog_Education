trait Trait<'a, T> {
    fn new(&self) -> Self;
    // signature uses generic type parameter
    fn func1(arg: T);

    // signature uses lifetime parameter
    fn func2(arg: &'a i32);

    // signature uses lifetime parameter and generic type parameter
    fn func3(arg: &'a T);
}

struct SomeType;

impl<'a> Trait<'a, i8> for SomeType {
    fn new(&self) -> SomeType {
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

fn main() {
    let some_type: SomeType = SomeType;
    <SomeType as Trait<'_, _>>::func1(8);
    <SomeType as Trait<'_, _>>::func2(&3200000);
    <SomeType as Trait<'_, _>>::func3(&100);
}
