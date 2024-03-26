use std::time::Duration;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    std::thread::sleep(Duration::from_millis(time));

    println!("Task {task} is done.");
}

async fn hello() {
    println!("test");
    for _ in 0..10 {
        println!("for hello");
    }
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 5000 * i));
        hello().await
    }
    futures::future::join_all(futures).await;
}
