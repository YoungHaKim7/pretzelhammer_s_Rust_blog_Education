use std::mem::size_of;

const WIDTH: usize = size_of::<&()>();
const DOUBLE_WIDTH: usize = 2 * WIDTH;

trait Trait {
    fn print(&self);
}

struct Struct;
struct Struct2;

impl Trait for Struct {
    fn print(&self) {
        println!("struct");
    }
}

impl Trait for Struct2 {
    fn print(&self) {
        println!("struct2");
    }
}

fn print_struct(s: &Struct) {
    // always prints "struct"
    // this is known at compile-time
    s.print();

    // single-width pointer
    assert_eq!(WIDTH, size_of::<&Struct>());
}

fn print_struct2(s2: &Struct2) {
    // always prints "struct"
    // this is known at compile-time
    s2.print();

    // single-width pointer
    assert_eq!(WIDTH, size_of::<&Struct2>());
}

fn print_trait(t: &dyn Trait) {
    // print "struct" or "struct2" ?
    //  this is unknows at compile-time
    t.print();

    // Rust has to check the pointer at run-time
    // to figure out whether to use Struct's
    // or Struct2's implementation of "print"
    // so the pointer has to be double --width
    assert_eq!(DOUBLE_WIDTH, size_of::<&dyn Trait>());
}

fn main() {
    // single-width pointer to data
    let s = &Struct;
    print_struct(s);

    // single-width pointer to data
    let s2 = &Struct2;
    print_struct2(s2); // prints "struct2"
                       //
                       //
                       // unsized coercion form Struct to dyn Trait
                       // double-width pointer to point to data AND Struct's vtable
    let t: &dyn Trait = &Struct;
    print_trait(t); // print "struct"

    // unsized coercion form Struct2 to dyn Trait
    // double-width pointer to point to data AND Struct2's vtable
    let t: &dyn Trait = &Struct2;
    print_trait(t); // print "struct2"
}
