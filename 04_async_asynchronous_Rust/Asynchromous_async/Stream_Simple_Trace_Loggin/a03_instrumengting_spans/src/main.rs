use tracing_subscriber::fmt::format::FmtSpan;

#[tracing::instrument]
async fn hello() {
    println!("Hello World~ instrument Spans");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    hello().await;
    Ok(())
}
