pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early dev.

async fn helll() {
    println!("hello snippet");
}

#[tokio::main]
async fn main() -> Result<()> {
    helll().await;
    Ok(())
}
