#![feature(negative_impls)]
struct Struct;

impl !Send for Struct {}

impl !Sync for Struct {}

// impl !Sized for Struct {}

fn main() {
    println!("Hello, world!");
}
