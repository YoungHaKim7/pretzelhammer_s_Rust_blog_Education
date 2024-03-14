use tokio::runtime::Builder;

async fn hello() {
    println!("tokio runtime builder hello");
}

fn main() {
    // YOU DON"T HAVE TO SPECIFY ANY OF THESE
    let runtime = Builder::new_multi_thread()
        .worker_threads(4) // 4 threads in the pool
        .thread_name("thread_namer") // Name the threads.
        // This helper names the "my-pool-#" for debugging assistance.
        .thread_stack_size(3 * 1024 * 1024) // You can set the stack size
        .global_queue_interval(61) // You can change how often the global work thread is checked
        .max_io_events_per_tick(1024) // You can limit the number of I/O events per tick
        // YOU CAN REPLACE THIS WITH INDIVIDUAL ENABLES PER FEATURE
        .enable_all()
        // Build the runtime
        .build()
        .unwrap();
    runtime.block_on(hello());
}
