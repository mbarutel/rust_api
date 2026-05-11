#[tokio::main]
async fn main() -> anyhow::Result<()> {
    conference_services_api::run().await
}
