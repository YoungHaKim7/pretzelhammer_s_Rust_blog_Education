use futures::executor::block_on;

async fn do_work() {
    println!("Hello, async world!");
}

fn main() {
    println!("Hello, sync wolrd!");
    block_on(do_work());
}
