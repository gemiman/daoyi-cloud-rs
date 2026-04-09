use daoyi_cloud_common::app;
use daoyi_module_demo::demo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run("demo", demo::create_router()).await
}
