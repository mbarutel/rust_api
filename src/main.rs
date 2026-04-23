#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rust_api::run().await
}
