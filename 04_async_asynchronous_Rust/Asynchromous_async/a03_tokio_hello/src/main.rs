async fn hello() {
    println!("Hello form async Tokio");
}

#[tokio::main]
async fn main() {
    hello().await;
}
