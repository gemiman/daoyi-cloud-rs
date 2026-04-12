use daoyi_cloud_common::app;
use daoyi_module_demo::demo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = demo::create_router();
    app::run("demo", router).await
}
