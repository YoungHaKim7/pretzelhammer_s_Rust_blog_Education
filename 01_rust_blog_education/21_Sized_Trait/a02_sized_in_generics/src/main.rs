// this generic fn
fn func<T>(t: T) {}

// ...desugars to...
fn func_desugars<T: Sized>(t: T) {}

// fn func<T: ?Sized>(t: T) {}
//

fn func02<T: ?Sized>(t: &T) {}

fn func03<T: ?Sized>(t: Box<T>) {}

fn main() {
    println!("Hello, world!");
}
