use std::fmt::Display;

// fn dynamic_thread_print(t: Box<dyn Display + Send + 'static>) {
fn dynamic_thread_print(t: Box<dyn Display + Send>) {
    std::thread::spawn(move || {
        println!("dynamic_thread_print: {}", t);
    })
    .join();
}

fn static_thread_print<T: Display + Send + 'static>(t: T) {
    std::thread::spawn(move || {
        println!("static_thread_print: {}", t);
    })
    .join();
}
fn main() {
    dynamic_thread_print(Box::new(10));
    static_thread_print(10);
    println!("Hello, world!");
}
