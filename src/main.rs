use daoyi_cloud_common::app;

pub mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run("server", api::create_router()).await
}
