use std::io::Write;

// with sugar
fn function() {}

// desugared
fn function2() -> () {}

fn main() {
    let a: () = {};
    let b: i32 = { 5 };
    // let c: () = { 5 };
    let c: u64 = { 5 };
    dbg!(a, b, c);
    function();
    function2();
}
