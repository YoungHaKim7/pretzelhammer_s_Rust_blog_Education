#[derive(Debug)]
struct NumRef<'a>(&'a i32);

impl<'a> NumRef<'a> {
    // my struct is generic over 'a so that means I need to annotate
    // my self paprameters with 'a too, right? (answer: no, not right)
    fn some_method(&mut self) {}

    // above line desugars to
    // fn some_method_desugred<'b>(&'b mut self) {}
    fn some_method_desugred(&mut self) {}
}

fn main() {
    let mut num_ref = NumRef(&5);
    num_ref.some_method(); // mutably borrows num_ref for the rest its lifetime
    num_ref.some_method();
    num_ref.some_method_desugred();
    num_ref.some_method_desugred();
    // num_ref.some_method(); // mutably borrows num_ref for the rest its lifetime
    println!("{:?}", num_ref);
    println!("{:?}", num_ref);
    println!("{:?}", num_ref);
}
