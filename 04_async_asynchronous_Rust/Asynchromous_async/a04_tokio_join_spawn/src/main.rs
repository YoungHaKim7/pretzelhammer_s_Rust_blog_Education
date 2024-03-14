use tokio::{join, task::JoinSet};

async fn double(n: i32) -> i32 {
    n * n
}

async fn hello() {
    println!("Hello form async tokio");

    // Use the tokio::join! macro
    let result: (i32, i32) = join!(double(2), double(3));
    println!("{result:?}");

    // You can still use futures join_all
    let futures = vec![double(2), double(3)];
    let result = futures::future::join_all(futures).await;
    println!("{result:?}");

    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(double(i));
    }
    while let Some(res) = set.join_next().await {
        println!("{res:?}");
    }
}
#[tokio::main]
async fn main() {
    hello().await;
}
