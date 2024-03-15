use tokio::{pin, select};

async fn my_async_fn() {
    // async logic here
    println!("my_async_fn logic");
}

#[tokio::main]
async fn main() {
    pin! {
        let future1 = my_async_fn();
        let future2 = my_async_fn();
    }

    select! {
        _ = &mut future1 => {}
        _ = &mut future2 => {}
    }
}
