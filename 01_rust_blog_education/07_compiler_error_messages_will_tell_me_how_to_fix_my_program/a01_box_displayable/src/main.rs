use std::fmt::Display;

fn box_displayable<T: Display + 'static>(t: T) -> Box<dyn Display> {
    Box::new(t)
}

fn return_first<'a>(a: &'a str, b: &'a str) -> &'a str {
    a
}

fn main() {
    dbg!(return_first("test", "good2"));
    println!("Hello, world!");
}
