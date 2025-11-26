use daoyi_gateway::run_gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_gateway().await
}
