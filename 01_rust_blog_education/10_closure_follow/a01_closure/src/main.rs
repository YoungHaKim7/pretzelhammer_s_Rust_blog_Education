fn function_name() {
    println!("Hello, world!");
}

fn function(x: &i32) -> &i32 {
    x
}
fn main() {
    let closure = |x: &i32| x;
    println!("{}", function(&3));
}
