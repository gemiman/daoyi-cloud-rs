use daoyi_cloud_common::app;

pub mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = api::create_router();
    app::run("server", router).await
}
