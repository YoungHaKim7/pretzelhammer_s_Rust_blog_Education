use std::sync::Mutex;

// region:    --- Modules

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early dev.

async fn foo(x: &Mutex<u32>) {
    let mut guard = x.lock().unwrap();
    *guard += 1;
    baz().await;
}

async fn baz() {
    println!("baz te");
}

async fn bar(x: &Mutex<u32>) {
    let mut guard = x.lock().unwrap();
    *guard += 1;
    drop(guard); // explicit drop
    baz().await;
}

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let mut x = 10;
    x += 1;
    println!("{}", x);
    let y = Mutex::new(30);
    bar(&y).await;
    Ok(())
}
