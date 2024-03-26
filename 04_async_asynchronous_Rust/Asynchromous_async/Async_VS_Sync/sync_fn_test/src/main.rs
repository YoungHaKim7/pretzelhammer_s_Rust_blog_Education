use std::time::Duration;

fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    std::thread::sleep(Duration::from_millis(time));

    println!("Task {task} is done.");
}

fn hello() {
    println!("test");
    for _ in 0..10 {
        println!("for hello");
    }
}

fn main() {
    println!("Hello, world!");

    for i in 0..5 {
        hello_delay(i, 5000 * i);
        hello()
    }
}
