#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().json().finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("This is an error!");
}
