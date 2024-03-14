use async_recursion::async_recursion;

#[async_recursion]
async fn fib(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1).await + fib(n - 2).await,
    }
}

#[tokio::main]
async fn main() {
    println!("fib(10) = {}", fib(10).await);
}
